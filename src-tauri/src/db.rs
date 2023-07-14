use rusqlite::{Connection, Result};
use std::fs;
use tauri::AppHandle;

const CURRENT_DB_VERSION: u32 = 1;

pub fn create_db(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");

    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("Videos.sqlite");

    let mut conn = Connection::open(sqlite_path)?;

    let mut user_pragma = conn.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut conn, existing_user_version)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS videos (
        id INTEGER PRIMARY KEY,
        path TEXT NOT NULL UNIQUE,
        favorite BOOLEAN NOT NULL
        );",
    )?;

    Ok(conn)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.commit()?;
    }

    Ok(())
}

pub fn insert_video(path: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut query = db.prepare("INSERT INTO videos (path, favorite) VALUES (?1, ?2)")?;

    query.execute([path, "0"])?;

    Ok(())
}

pub fn get_videos(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT path FROM videos")?;
    let video_iter = stmt.query_map([], |row| Ok(row.get(0)?))?;

    let mut videos = Vec::new();
    for video in video_iter {
        videos.push(video?);
    }

    Ok(videos)
}

pub fn delete_video(path: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut query = db.prepare("DELETE FROM videos WHERE path = ?1")?;

    query.execute([path])?;

    Ok(())
}

pub fn delete_selected_videos(paths: Vec<String>, db: &Connection) -> Result<(), rusqlite::Error> {
    for path in paths {
        delete_video(&path, db)?;
    }

    Ok(())
}

pub fn get_favorite_videos(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = db.prepare("SELECT path FROM videos WHERE favorite = ?1")?;
    let favorite_path_iter = stmt.query_map(["1"], |row| Ok(row.get(0)?))?;

    let mut paths = Vec::new();

    for path in favorite_path_iter {
        paths.push(path?)
    }

    Ok(paths)
}

pub fn set_favorite_video(path: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut query = db.prepare("UPDATE videos SET favorite = 1 WHERE path = ?1")?;
    query.execute([path])?;

    Ok(())
}

pub fn unset_favorite_video(path: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut query = db.prepare("UPDATE videos SET favorite = 0 WHERE path = ?1")?;
    query.execute([path])?;

    Ok(())
}
