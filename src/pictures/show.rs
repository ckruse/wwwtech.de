use actix_files as fs;
use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use serde::Deserialize;
use std::path::Path;

use crate::models::Picture;
use crate::uri_helpers::picture_img_uri;
use crate::utils::image_base_path;
use crate::DbPool;

use super::{actions, ImageTypes};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Deserialize)]
pub struct TypeParams {
    #[serde(rename = "type")]
    pic_type: Option<ImageTypes>,
}

#[get("/pictures/{id}.{ext}")]
pub async fn show_img(
    pool: web::Data<DbPool>,
    info: web::Path<(i32, String)>,
    pic_type: web::Query<TypeParams>,
) -> Result<fs::NamedFile, Error> {
    let (id, _ext) = info.into_inner();
    let pic_type = match pic_type.into_inner().pic_type {
        Some(val) => val,
        None => ImageTypes::Thumbnail,
    };

    let path_part = match pic_type {
        ImageTypes::Large => "large",
        ImageTypes::Original => "original",
        ImageTypes::Thumbnail => "thumbnail",
    };

    let picture = web::block(move || {
        let conn = pool.get()?;
        actions::get_picture(id, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut path = format!(
        "{}/{}/{}/{}",
        image_base_path(),
        picture.id,
        path_part,
        picture.image_file_name
    );

    if !Path::new(&path).exists() {
        path = format!(
            "{}/{}/original/{}",
            image_base_path(),
            picture.id,
            picture.image_file_name
        );
    }

    Ok(fs::NamedFile::open(path)?)
}

#[derive(Template)]
#[template(path = "pictures/show.html.jinja")]
struct Show<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    picture: &'a Picture,
    index: bool,
    atom: bool,
    home: bool,
    picture_type: &'a str,
}

#[get("/pictures/{id}")]
pub async fn show(ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let picture = web::block(move || {
        let conn = pool.get()?;
        actions::get_picture(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Show {
        title: Some(&format!("Picture #{}: {}", picture.id, picture.title)),
        page_type: None,
        page_image: Some(&picture_img_uri(&picture)),
        body_id: Some("pictures-show"),
        logged_in: ident.identity().is_some(),
        picture: &picture,
        index: false,
        atom: false,
        home: false,
        picture_type: "large",
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
