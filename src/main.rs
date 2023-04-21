#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate dotenv;

use crate::invoke::*;
use std::env;
use tauri::Manager;

mod invoke;
mod models;
mod tray;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub struct Store {
    pub netease_server: String,
    pub api_server: String,
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
            get_music_detail,
            get_music_url,
            check_server,
            check,
            login,
            register,
            get_playlist_all_music,
            get_playlist_detail,
            search,
            get_album_detail,
            get_artist_common_detail,
            get_artist_all_songs,
            get_artist_all_albums,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
