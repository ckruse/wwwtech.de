use std::collections::HashMap;

use actix_web::web;
use chrono::Duration;

use crate::{caching_middleware, models::NewDeafie, multipart::MultipartField};

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
                .service(show::show_img)
                .service(show::show),
        );
}

pub fn form_from_params(
    params: &HashMap<String, MultipartField>,
    author_id: i32,
    filename: Option<String>,
    content_type: Option<&str>,
) -> NewDeafie {
    NewDeafie {
        author_id: Some(author_id),
        title: match params.get("title") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        slug: match params.get("slug") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        guid: match params.get("guid") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        image_name: filename,
        image_content_type: match content_type {
            Some(s) => Some(s.to_owned()),
            _ => None,
        },
        excerpt: match params.get("excerpt") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        body: match params.get("body") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        published: match params.get("published") {
            Some(MultipartField::Form(v)) => v == "true",
            _ => false,
        },
        ..Default::default()
    }
}
