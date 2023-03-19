#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::invoke::*;
use tauri::{SystemTrayEvent, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
use tauri:: Manager;

mod invoke;
mod models;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                if !window.is_visible().unwrap() {
                    window.show().unwrap();
                }
                println!("System tray received a left click");
            },
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("System tray received a right click");
            },
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("System tray received a double click");
            },
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_music_detail,
            get_music_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
