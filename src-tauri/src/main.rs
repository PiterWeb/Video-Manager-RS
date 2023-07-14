// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod state;

use crate::state::AppState;
use ffmpeg_sidecar::command::ffmpeg_is_installed;
use tauri::{Manager, State};

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            commands::insert_video,
            commands::get_videos,
            commands::scan_folder,
            commands::delete_selected_videos,
            commands::get_favorite_videos,
            commands::set_favorite_video,
            commands::unset_favorite_video
        ])
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();

            let db = db::create_db(&handle)?;
            *app_state.db.lock().unwrap() = Some(db);

            if !ffmpeg_is_installed() {
                ffmpeg_sidecar::download::auto_download().unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
