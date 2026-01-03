// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager, WebviewWindow};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL to load (e.g., https://google.com or http://localhost:3000)
    #[arg(short, long)]
    pub url: Option<String>,

    /// Window width in pixels
    #[arg(long)]
    pub width: Option<f64>,

    /// Window height in pixels
    #[arg(long)]
    pub height: Option<f64>,

    /// Start in fullscreen mode
    #[arg(long)]
    pub fullscreen: bool,

    /// Start minimized
    #[arg(long)]
    pub minimized: bool,

    /// Comma-separated list of allowed domains for navigation (e.g., "google.com,github.com")
    #[arg(long, value_delimiter = ',')]
    pub allowed_domains: Option<Vec<String>>,

    /// Comma-separated list of domains allowed to access the native API
    #[arg(long, value_delimiter = ',')]
    pub api_domains: Option<Vec<String>>,

    /// Disable window resizing
    #[arg(long)]
    pub no_resizable: bool,

    /// Remove window frame (decorations)
    #[arg(long)]
    pub frameless: bool,
}

#[tauri::command]
fn greet(name: &str, window: WebviewWindow, app_handle: tauri::AppHandle) -> Result<String, String> {
    // We can access state here if we inject it, or just use global args if we stored them in state.
    // However, invoke handler cannot easily see main's args unless we manage state.
    // For now, let's keep the greeting simple or implement the check.
    // To implement the check, we need to pass the allowed API domains to the app state.
    
    // For now, valid implementation of a check would look like this:
    let state = app_handle.state::<AppState>();
    
    if let Some(api_domains) = &state.api_domains {
        let current_url = window.url().map_err(|e| e.to_string())?;
        let host = current_url.host_str().unwrap_or("");
        
        let allowed = api_domains.iter().any(|d| host.ends_with(d));
        if !allowed {
             return Err(format!("API access denied for domain: {}", host));
        }
    }

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

struct AppState {
    api_domains: Option<Vec<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_app(Args::parse())
}

pub fn run_app(args: Args) {
    let api_domains = args.api_domains.clone();
    let allowed_domains = args.allowed_domains.clone();
    
    // Default URL: fallback to usage of `tauri::Url` or just empty string/local index.
    // If we don't provide a URL to the builder, it defaults to tauri://localhost (index.html).
    // Let's use the provided URL or fallback.
    let start_url = args.url.as_deref().unwrap_or("tauri://localhost");
    let url = tauri::Url::parse(start_url).unwrap_or_else(|_| tauri::Url::parse("tauri://localhost").unwrap());

    tauri::Builder::default()
        .manage(AppState { api_domains })
        .setup(move |app| {
            // Create the main window programmatically
            let mut builder = tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::External(url));
            
            builder = builder.title("Tauri App");
            
            if let Some(width) = args.width {
                if let Some(height) = args.height {
                     builder = builder.inner_size(width, height);
                } else {
                     // Default height 600 if only width is given
                     builder = builder.inner_size(width, 600.0);
                }
            } else if let Some(height) = args.height {
                 // Default width 800 if only height is given
                 builder = builder.inner_size(800.0, height);
            }

            if args.fullscreen {
                builder = builder.fullscreen(true);
            }

            if args.no_resizable {
                builder = builder.resizable(false);
            }

            if args.frameless {
                builder = builder.decorations(false);
            }

            // Navigation Lock
            if let Some(domains) = allowed_domains {
                builder = builder.on_navigation(move |url| {
                     let host = url.host_str().unwrap_or("");
                     let allowed = domains.iter().any(|d| host.ends_with(d));
                     if allowed {
                         true
                     } else {
                         println!("Blocked navigation to: {}", url);
                         false
                     }
                });
            }

            let window = builder.build().unwrap();

            if args.minimized {
                let _ = window.minimize();
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
