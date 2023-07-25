use actix_files as fs;

pub fn handle() -> fs::Files {
    fs::Files::new("/static", "")
}