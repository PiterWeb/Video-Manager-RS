use crate::db;
use crate::state::ServiceAccess;
use tauri::AppHandle;

use walkdir::WalkDir;

#[tauri::command]
pub fn insert_video(app_handle: AppHandle, path: &str) -> String {
    const SUPPORTED_VIDEO_EXTENSIONS: [&str; 3] = ["mp4", "mkv", "avi"];

    let is_supported = SUPPORTED_VIDEO_EXTENSIONS
        .iter()
        .any(|ext| path.ends_with(ext));

    if !is_supported {
        return format!("error inserting video: unsupported file extension");
    }

    match app_handle.db(|db| db::insert_video(path, db)) {
        Ok(_) => "inserted video".to_string(),

        Err(e) => {
            format!("error inserting video: {}", e)
        }
    }
}

#[tauri::command]
pub fn get_videos(app_handle: AppHandle) -> Vec<String> {
    match app_handle.db(|db| db::get_videos(db)) {
        Ok(videos) => {
            return videos;
        }

        Err(e) => {
            println!("error getting videos: {}", e);
            return vec![];
        }
    }
}

#[tauri::command]
pub fn scan_folder(app_handle: AppHandle, path: &str) {
    const SUPPORTED_VIDEO_EXTENSIONS: [&str; 3] = ["mp4", "mkv", "avi"];
    let walker = WalkDir::new(path).into_iter();

    for entry in walker.filter_map(|e| e.ok()).filter(|e| {
        let is_file = e.file_type().is_file();
        let is_supported = SUPPORTED_VIDEO_EXTENSIONS
            .iter()
            .any(|ext| e.path().to_str().unwrap().ends_with(ext));

        return is_file && is_supported;
    }) {
        insert_video(app_handle.clone(), entry.path().to_str().unwrap());
    }
}

#[tauri::command]
pub fn delete_selected_videos(app_handle: AppHandle, paths: Vec<String>) -> String {
    match app_handle.db(|db| db::delete_selected_videos(paths, db)) {
        Ok(_) => {
            format!("deleted videos")
        }

        Err(e) => {
            format!("error deleting videos: {}", e)
        }
    }
}
