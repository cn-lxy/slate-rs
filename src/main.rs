#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::invoke::*;
use tauri::Manager;

mod invoke;
mod models;
mod tray;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            // listen to the `event-name`.
            let _id = app.listen_global("event-name", |event| {
                println!("got  event-name with payload {:?}", event.payload());
            });
            let _ = app.listen_global("music-name", move |event| {
                let _new_nappame = event.payload().unwrap();
            });
            // emit the `event-name` event to all webview windows on the frontend.
            app.emit_all(
                "Sevet-name",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();

            Ok(())
        })
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_music_detail,
            get_music_url,
            check_server,
            check,
            login,
            register,
            get_hot_music_list,
            get_playlist_detail,
            search,
            get_album_detail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
