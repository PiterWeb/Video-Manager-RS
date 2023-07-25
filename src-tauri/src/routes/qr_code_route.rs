use actix_web::{web, HttpRequest, Result};

pub async fn handler(info: web::Path<String>, _req: HttpRequest) -> Result<String> {
    let info = info.into_inner();

    Ok(format!("Path {}", info))
}
