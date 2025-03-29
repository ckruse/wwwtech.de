use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_typed_multipart::TypedMultipart;

use super::{PictureData, actions};
use crate::errors::AppError;
use crate::models::{NewPicture, generate_pictures};
use crate::posse::mastodon::post_picture;
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "pictures/new.html.j2")]
pub struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: NewPicture,
    error: Option<String>,
}

pub async fn new() -> Result<Response, AppError> {
    let html = New {
        lang: "en",
        title: Some("New picture"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: None,
        form_data: NewPicture {
            posse: true,
            show_in_index: true,
            lang: "en".to_owned(),
            ..Default::default()
        },
    }
    .render()?;

    Ok(Html(html).into_response())
}

pub async fn create(
    auth: AuthSession,
    State(state): State<AppState>,
    TypedMultipart(data): TypedMultipart<PictureData>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut conn = state.pool.acquire().await?;

    let filename = data.picture.metadata.file_name.clone().unwrap_or("img.jpg".to_string());
    let content_type = new_mime_guess::from_path(&filename)
        .first_raw()
        .unwrap_or("image/jpeg")
        .to_owned();

    let values = NewPicture {
        title: data.title,
        author_id: Some(user.id),
        alt: data.alt,
        in_reply_to: data.in_reply_to,
        lang: data.lang,
        posse: data.posse,
        show_in_index: data.show_in_index,
        content: data.content,

        image_file_name: Some(filename),
        image_content_type: Some(content_type),

        posse_visibility: data.posse_visibility,
        content_warning: data.content_warning,
        ..Default::default()
    };

    let f = tokio::fs::File::from_std(
        data.picture
            .contents
            .as_file()
            .try_clone()
            .map_err(|e| AppError::InternalError(format!("could not clone file handle: {}", e)))?,
    );

    match actions::create_picture(&values, Some(f), &mut conn).await {
        Ok(picture) => {
            let uri = picture_uri(&picture);

            tokio::task::spawn_blocking(move || {
                let uri = picture_uri(&picture);
                let _ = generate_pictures(&picture);
                let _ = send_mentions(&uri);

                if picture.posse {
                    tokio::task::spawn(async move {
                        let _ = post_picture(&picture).await;
                    });
                }
            });

            Ok(Redirect::to(&uri).into_response())
        }

        Err(error) => {
            let html = New {
                lang: "en",
                title: Some("New picture"),
                page_type: None,
                page_image: None,
                body_id: None,
                logged_in: true,
                form_data: values,
                error: Some(error.to_string()),
            }
            .render()?;

            Ok((StatusCode::UNPROCESSABLE_ENTITY, Html(html)).into_response())
        }
    }
}
