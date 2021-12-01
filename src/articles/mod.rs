use actix_web::web;

pub mod actions;
pub mod index;
pub mod show;

static PER_PAGE: i64 = 10;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index).service(index::index_atom).service(show::show);
}
