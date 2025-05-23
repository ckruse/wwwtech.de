use std::path::Path;

use askama::Template;
use axum::body::Body;
use axum::extract::{Path as EPath, Query, State};
use axum::http::header;
use axum::response::{Html, IntoResponse, Response};
use sqlx::PgConnection;
use tokio_util::io::ReaderStream;

use super::{ImageTypes, TypeParams, actions};
use crate::errors::AppError;
use crate::models::Picture;
use crate::uri_helpers::*;
use crate::utils::image_base_path;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "pictures/show.html.j2")]
pub struct Show<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<String>,
    body_id: Option<&'a str>,
    logged_in: bool,

    picture: Picture,
    index: bool,
    atom: bool,
    home: bool,
    picture_type: &'a str,
}

pub async fn show(
    auth: AuthSession,
    State(state): State<AppState>,
    EPath(info): EPath<String>,
    pic_type: Query<TypeParams>,
) -> Result<Response, AppError> {
    let parts = info.rsplit_once('.');

    if let Some((id, suffix)) = parts {
        let id = id
            .parse()
            .map_err(|e| AppError::InternalError(format!("error parsing id: {}", e)))?;

        show_img(state, id, suffix, pic_type).await
    } else {
        let id = info
            .parse()
            .map_err(|e| AppError::InternalError(format!("error parsing id: {}", e)))?;
        show_post(state, id, auth.user.is_some()).await
    }
}

pub async fn show_post(state: AppState, id: i32, logged_in: bool) -> Result<Response, AppError> {
    let mut conn = state.pool.acquire().await?;
    let picture = get_image(id, &state, &mut conn).await?;

    let html = Show {
        lang: "en",
        title: Some(picture.title.clone()),
        page_type: None,
        page_image: Some(picture_img_uri(&picture, None)),
        body_id: Some("pictures-show"),
        logged_in,
        picture,
        index: false,
        atom: false,
        home: false,
        picture_type: "large",
    }
    .render()?;

    Ok(Html(html).into_response())
}

async fn get_image(id: i32, state: &AppState, conn: &mut PgConnection) -> Result<Picture, AppError> {
    let picture = match state.picture_cache.get(&id).await {
        Some(picture) => picture,
        None => {
            let picture = actions::get_picture(id, conn).await?;
            state.picture_cache.insert(id, picture.clone()).await;

            picture
        }
    };

    Ok(picture)
}

pub async fn show_img(state: AppState, id: i32, ext: &str, pic_type: Query<TypeParams>) -> Result<Response, AppError> {
    let pic_type = match pic_type.pic_type {
        Some(val) => val,
        None => ImageTypes::Original,
    };

    let path_part = match pic_type {
        ImageTypes::Large => "large",
        ImageTypes::Original => "original",
        ImageTypes::Thumbnail => "thumbnail",
    };

    let mut conn = state.pool.acquire().await?;
    let picture = actions::get_picture(id, &mut conn).await?;

    let mut path = format!("{}/{}/{}/{}", image_base_path(), picture.id, path_part, picture.image_file_name);

    if !Path::new(&path).exists() {
        path = format!("{}/{}/original/{}", image_base_path(), picture.id, picture.image_file_name);
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
