use actix_web::web;

pub mod actions;

pub mod index;
pub mod show;

static PER_PAGE: i64 = 25;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index).service(show::show);
}
