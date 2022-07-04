use actix_web::web;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{caching_middleware, models::NewPicture, multipart::MultipartField};

pub mod actions;

pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 60;

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageTypes {
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "original")]
    Original,
}

#[derive(Deserialize)]
pub struct TypeParams {
    #[serde(rename = "type")]
    pub pic_type: Option<ImageTypes>,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index_atom)
        .service(new::create)
        .service(new::new)
        .service(edit::edit)
        .service(edit::update)
        .service(delete::delete)
        .service(
            web::scope("/pictures")
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
    metadata: &Option<(String, String, i32)>,
) -> NewPicture {
    let (filename, content_type, size) = match metadata {
        Some((filename, content_type, size)) => {
            (Some(filename.clone()), Some(content_type.clone()), Some(size.clone()))
        }
        None => (None, None, None),
    };

    NewPicture {
        author_id: Some(author_id),
        title: match params.get("title") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        alt: match params.get("alt") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        lang: match params.get("lang") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        in_reply_to: match params.get("in_reply_to") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        posse: match params.get("posse") {
            Some(MultipartField::Form(v)) => v == "true",
            _ => false,
        },
        show_in_index: match params.get("show_in_index") {
            Some(MultipartField::Form(v)) => v == "true",
            _ => false,
        },
        content: match params.get("content") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        image_file_name: filename,
        image_content_type: content_type,
        image_file_size: size,
        ..Default::default()
    }
}
