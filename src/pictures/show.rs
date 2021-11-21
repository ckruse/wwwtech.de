use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use serde::Deserialize;

use crate::uri_helpers::picture_img_uri;
use crate::utils::image_base_path;
use crate::DbPool;

use super::{actions, ImageTypes};

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
        actions::get_picture(id, true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let path = format!(
        "{}/{}/{}/{}",
        image_base_path(),
        picture.id,
        path_part,
        picture.image_file_name
    );

    Ok(fs::NamedFile::open(path)?.set_status_code(StatusCode::NOT_FOUND))
}

#[get("/pictures/{id}")]
pub async fn show(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let picture = web::block(move || {
        let conn = pool.get()?;
        actions::get_picture(id.into_inner(), true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut ctx = tera::Context::new();
    ctx.insert("picture", &picture);
    ctx.insert("title", &format!("Picture #{}: {}", picture.id, picture.title));
    ctx.insert("body_id", "pictures-show");
    ctx.insert("type", "large");
    ctx.insert("index", &false);
    ctx.insert("page_image", &picture_img_uri(&picture));

    let s = tmpl
        .render("pictures/show.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
