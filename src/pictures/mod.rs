use actix_web::web;
use serde::{Deserialize, Serialize};

pub mod actions;

pub mod index;
pub mod show;

static PER_PAGE: i64 = 36;

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageTypes {
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "original")]
    Original,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index).service(show::show_img).service(show::show);
}
