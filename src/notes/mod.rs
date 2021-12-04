use actix_web::web;

pub mod actions;

pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 25;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(index::index_atom)
        .service(new::new)
        .service(new::create)
        .service(show::show);
}
