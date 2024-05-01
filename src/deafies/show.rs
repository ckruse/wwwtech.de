use std::path::Path;

use askama::Template;
use axum::{
    body::Body,
    extract::{Path as EPath, Query, State},
    http::header,
    response::{IntoResponse, Response},
};
use regex::Regex;
use tokio_util::io::ReaderStream;

use super::actions;
use crate::{
    errors::AppError,
    models::Deafie,
    pictures::{ImageTypes, TypeParams},
    uri_helpers::*,
    utils as filters,
    utils::deafie_image_base_path,
    AppState, AuthSession,
};

#[derive(Template)]
#[template(path = "deafies/show.html.jinja")]
pub struct Show<'a> {
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

pub async fn show(
    auth: AuthSession,
    State(state): State<AppState>,
    EPath((year, month, slug)): EPath<(i32, String, String)>,
    Query(pic_type): Query<TypeParams>,
) -> Result<Response, AppError> {
    let logged_in = auth.user.is_some();
    let guid = format!("{}/{}/{}", year, month, slug);
    let pic_type = match pic_type.pic_type {
        Some(val) => val,
        None => ImageTypes::Original,
    };

    let rx = Regex::new(r"\.(\w+)$").map_err(|e| AppError::InternalError(e.to_string()))?;
    if rx.is_match(&slug) {
        let captures = rx
            .captures(&slug)
            .ok_or_else(|| AppError::InternalError("Invalid regex".to_string()))?;

        let ext = captures
            .get(1)
            .ok_or_else(|| AppError::InternalError("Invalid regex".to_string()))?;

        let ext = ext.as_str();
        let guid = guid.replace(format!(".{}", ext).as_str(), "");

        show_img(state, guid, logged_in, pic_type, ext).await
    } else {
        show_post(state, logged_in, guid).await
    }
}

pub async fn show_img(
    state: AppState,
    guid: String,
    logged_in: bool,
    pic_type: ImageTypes,
    ext: &str,
) -> Result<Response, AppError> {
    let path_part = match pic_type {
        ImageTypes::Large => "large",
        ImageTypes::Original => "original",
        ImageTypes::Thumbnail => "thumbnail",
    };

    let mut conn = state.pool.acquire().await?;
    let deafie = actions::get_deafie_by_slug(&guid, !logged_in, &mut conn).await?;

    let image_name = match deafie.image_name {
        Some(image_name) => Ok(image_name),
        None => Err(AppError::NotFound("article has no image".to_owned())),
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

    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err(AppError::NotFound(format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);

    let ctype = match ext {
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    };

    Ok(([(header::CONTENT_TYPE, ctype)], Body::from_stream(stream)).into_response())
}

pub async fn show_post(state: AppState, logged_in: bool, guid: String) -> Result<Response, AppError> {
    let mut conn = state.pool.acquire().await?;
    let deafie = actions::get_deafie_by_slug(&guid, !logged_in, &mut conn).await?;

    let uri = deafie_img_uri(&deafie, None);
    let page_image = if deafie.image_name.is_some() {
        Some(uri.as_str())
    } else {
        None
    };

    Ok(Show {
        lang: "de",
        title: Some(&deafie.title),
        page_type: Some("blog"),
        page_image,
        body_id: None,
        logged_in,
        deafie: &deafie,
        index: false,
        atom: false,
    }
    .into_response())
}
