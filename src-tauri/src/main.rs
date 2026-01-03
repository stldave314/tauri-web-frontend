// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to load (e.g., https://google.com or http://localhost:3000)
    #[arg(short, long)]
    url: Option<String>,
}

fn main() {
    let args = Args::parse();
    tauri_app_lib::run_app(args.url)
}
