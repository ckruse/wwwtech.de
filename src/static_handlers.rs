use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::{get, Result};

use crate::utils;

#[get("/favicon.ico")]
pub async fn favicon() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/favicon.ico");

    Ok(fs::NamedFile::open(path)?)
}

#[get("/robots.txt")]
pub async fn robots_txt() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/robots.txt");

    Ok(fs::NamedFile::open(path)?)
}

#[get("/A99A9D73.asc")]
pub async fn gpgkey() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/A99A9D73.asc");

    Ok(fs::NamedFile::open(path)?)
}

#[get("/humans.txt")]
pub async fn humans_txt() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/humans.txt");

    Ok(fs::NamedFile::open(path)?)
}

pub async fn p404() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("404.html");

    Ok(fs::NamedFile::open(path)?.set_status_code(StatusCode::NOT_FOUND))
}
