use actix_files as fs;
use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use std::path::Path;

use crate::pictures::{ImageTypes, TypeParams};
use crate::DbPool;

use super::actions;
use crate::models::Deafie;
use crate::utils::deafie_image_base_path;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "deafies/show.html.jinja")]
struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    deafie: &'a Deafie,
    index: bool,
    atom: bool,
}

#[get("/{year}/{month}/{slug}.{ext}")]
pub async fn show_img(
    ident: Identity,
    pool: web::Data<DbPool>,
    path: web::Path<(i32, String, String, String)>,
    pic_type: web::Query<TypeParams>,
) -> Result<fs::NamedFile, Error> {
    let logged_in = ident.identity().is_some();
    let (year, month, slug, _ext) = path.into_inner();
    let guid = format!("{}/{}/{}", year, month, slug);

    let pic_type = match pic_type.into_inner().pic_type {
        Some(val) => val,
        None => ImageTypes::Original,
    };

    let path_part = match pic_type {
        ImageTypes::Large => "large",
        ImageTypes::Original => "original",
        ImageTypes::Thumbnail => "thumbnail",
    };

    let deafie = web::block(move || {
        let conn = pool.get()?;
        actions::get_deafie_by_slug(&guid, !logged_in, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let image_name = match deafie.image_name {
        Some(image_name) => Ok(image_name),
        None => Err(error::ErrorInternalServerError("article has no image")),
    }?;

    let mut path = format!(
        "{}/{}/{}/{}",
        deafie_image_base_path(),
        deafie.id,
        path_part,
        image_name
    );

    if !Path::new(&path).exists() {
        path = format!("{}/{}/original/{}", deafie_image_base_path(), deafie.id, image_name);
    }

    Ok(fs::NamedFile::open(path)?)
}

#[get("/{year}/{month}/{slug}")]
pub async fn show(
    ident: Identity,
    pool: web::Data<DbPool>,
    path: web::Path<(i32, String, String)>,
) -> Result<HttpResponse, Error> {
    let logged_in = ident.identity().is_some();
    let (year, month, slug) = path.into_inner();
    let guid = format!("{}/{}/{}", year, month, slug);

    let deafie = web::block(move || {
        let conn = pool.get()?;
        actions::get_deafie_by_slug(&guid, !logged_in, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Show {
        lang: "de",
        title: Some(&deafie.title.clone()),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in,
        deafie: &deafie,
        index: false,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
