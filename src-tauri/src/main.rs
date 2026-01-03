// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

fn main() {
    let args = tauri_app_lib::Args::parse();
    tauri_app_lib::run_app(args)
}
