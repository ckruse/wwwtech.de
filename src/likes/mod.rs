use actix_web::web;

pub mod actions;

pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 50;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(index::index_atom)
        .service(new::new)
        .service(new::create)
        .service(edit::edit)
        .service(edit::update)
        .service(delete::delete)
        .service(show::show);
}
