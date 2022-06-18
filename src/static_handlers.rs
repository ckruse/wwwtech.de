use actix_files as fs;
use actix_web::http::{Method, StatusCode};
use actix_web::{get, Either, HttpResponse, Responder, Result};
use std::env;

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

#[get("/.well-known/security.txt")]
pub async fn well_known_security_txt() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/security.txt");

    Ok(fs::NamedFile::open(path)?)
}

#[get("/security.txt")]
pub async fn security_txt() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/security.txt");

    Ok(fs::NamedFile::open(path)?)
}

#[get("/.well-known/keybase.txt")]
pub async fn keybase_txt() -> Result<fs::NamedFile> {
    let path = env::var("KEYBASE_TXT").expect("KEYBASE_TXT is not set");

    Ok(fs::NamedFile::open(path)?)
}

pub async fn p404(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let mut path = utils::static_path();
            path.push_str("404.html");

            let file = fs::NamedFile::open(path)?
                .customize()
                .with_status(StatusCode::NOT_FOUND);

            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}
