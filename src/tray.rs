use tauri::{SystemTray, SystemTrayMenuItem, SystemTrayMenu, CustomMenuItem, SystemTrayEvent};
use tauri::AppHandle;
use tauri::Manager;

pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let previous = CustomMenuItem::new("previous", "上一首");
    let next = CustomMenuItem::new("next", "下一首");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(previous)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(next);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    system_tray
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
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
                },
                "previous" => {
                    println!("上一首");
                    app.emit_all("event-previous", "previous music").unwrap();
                },
                "next" => {
                    println!("下一首");
                    app.emit_all("event-next", "next music").unwrap();
                },
                _ => {}
            }
        },
        _ => {},
    }
}