use actix_web::web;
use chrono::Duration;

use crate::caching_middleware;

pub mod actions;
pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 10;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index_atom)
        .service(new::new)
        .service(new::create)
        .service(edit::edit)
        .service(edit::update)
        .service(delete::delete)
        .service(
            web::scope("/deaf-dog-training")
                .wrap(caching_middleware::Caching {
                    duration: Duration::hours(1),
                })
                .service(index::index)
                .service(show::show),
        );
}
