#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::invoke::*;

mod invoke;
mod models;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_music_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
