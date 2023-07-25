// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod routes;
mod state;

use crate::state::AppState;
use actix_web::{rt, web, App, HttpServer};
use ffmpeg_sidecar::command::ffmpeg_is_installed;
use tauri::{Manager, State};

fn main() {
    std::thread::spawn(move || {
        let rt = rt::Runtime::new().unwrap();
        rt.block_on(
            HttpServer::new(|| {
                App::new().service(routes::static_files::handle()).route(
                    "/qr/{filepath}",
                    web::get().to(routes::qr_code_route::handler),
                )
            })
            .bind(("127.0.0.1", 8080))?
            .run(),
        )
    });

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
