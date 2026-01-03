// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_app(None)
}

pub fn run_app(target_url: Option<String>) {
    tauri::Builder::default()
        .setup(move |app| {
            if let Some(url) = target_url {
                let main_window = app.get_webview_window("main").unwrap();
                main_window.navigate(tauri::Url::parse(&url).unwrap())?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
