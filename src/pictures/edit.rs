use askama::Template;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum_typed_multipart::TypedMultipart;

use super::{PictureData, actions};
use crate::errors::AppError;
use crate::models::{NewPicture, Picture, generate_pictures};
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "pictures/edit.html.j2")]
pub struct Edit<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    picture: Picture,
    form_data: NewPicture,
    error: Option<String>,
}

pub async fn edit(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Edit<'static>, AppError> {
    let mut conn = state.pool.acquire().await?;

    let picture = actions::get_picture(id, &mut conn).await?;

    Ok(Edit {
        lang: "en",
        title: Some(format!("Edit picture #{}", picture.id)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        form_data: NewPicture {
            author_id: None,
            title: picture.title.clone(),
            alt: picture.alt.clone(),
            in_reply_to: picture.in_reply_to.clone(),
            lang: picture.lang.clone(),
            posse: picture.posse,
            show_in_index: picture.show_in_index,
            content: Some(picture.content.clone()),
            posse_visibility: picture.posse_visibility.clone(),
            content_warning: picture.content_warning.clone(),
            ..Default::default()
        },

        picture,
        error: None,
    })
}

pub async fn update(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    TypedMultipart(data): TypedMultipart<PictureData>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut conn = state.pool.acquire().await?;

    let filename = data
        .picture
        .metadata
        .file_name
        .clone()
        .unwrap_or_else(|| "img.jpg".to_string());
    let filename = if filename.is_empty() { None } else { Some(filename) };

    let content_type = filename.as_ref().map(|filename| {
        new_mime_guess::from_path(filename)
            .first_raw()
            .unwrap_or("image/jpeg")
            .to_owned()
    });

    let picture = actions::get_picture(id, &mut conn).await?;

    let f = if filename.is_some() {
        Some(tokio::fs::File::from_std(
            data.picture
                .contents
                .as_file()
                .try_clone()
                .map_err(|e| AppError::InternalError(format!("could not clone file handle: {}", e)))?,
        ))
    } else {
        None
    };

    let values = NewPicture {
        title: data.title,
        author_id: Some(user.id),
        alt: data.alt,
        in_reply_to: data.in_reply_to,
        lang: data.lang,
        posse: data.posse,
        show_in_index: data.show_in_index,
        content: data.content,

        image_file_name: filename,
        image_content_type: content_type,

        posse_visibility: data.posse_visibility,
        content_warning: data.content_warning,
        ..Default::default()
    };

    match actions::update_picture(&picture, &values, f, &mut conn).await {
        Ok(picture) => {
            state.picture_cache.insert(picture.id, picture.clone()).await;
            let uri = picture_uri(&picture);

            tokio::task::spawn_blocking(move || {
                let uri = picture_uri(&picture);
                let _ = generate_pictures(&picture);
                let _ = send_mentions(&uri);
            });

            Ok(Redirect::to(&uri).into_response())
        }

        Err(error) => Ok(Edit {
            lang: "en",
            title: Some(format!("Edit picture #{}", picture.id)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            picture,
            form_data: values,
            error: Some(error.to_string()),
        }
        .into_response()),
    }
}
