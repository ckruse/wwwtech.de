use actix_web::web;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct PageParams {
    p: Option<i64>,
}

pub fn base_path() -> String {
    env::var("BASE_PATH").unwrap_or(env::var("CARGO_MANIFEST_DIR").unwrap())
}

pub fn static_path() -> String {
    let mut str = base_path();
    str.push_str("/static/");

    str
}

pub fn get_page(page: &web::Query<PageParams>) -> i64 {
    let mut p = match page.p {
        Some(page) => page,
        None => 0,
    };

    if p < 0 {
        p = 0;
    }

    p
}
