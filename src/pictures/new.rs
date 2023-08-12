use askama::Template;
use axum::{
    extract::{Extension, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_typed_multipart::TypedMultipart;

use super::{actions, PictureData};
use crate::{
    errors::AppError,
    models::Author,
    models::{generate_pictures, NewPicture},
    posse::mastodon::post_picture,
    uri_helpers::*,
    utils as filters,
    webmentions::send::send_mentions,
    AppState,
};

#[derive(Template)]
#[template(path = "pictures/new.html.jinja")]
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

pub async fn new() -> New<'static> {
    New {
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
}

pub async fn create(
    Extension(user): Extension<Author>,
    State(state): State<AppState>,
    TypedMultipart(data): TypedMultipart<PictureData>,
) -> Result<Response, AppError> {
    let mut conn = state.pool.acquire().await?;

    let filename = data
        .picture
        .as_ref()
        .map(|f| f.metadata.file_name.clone().unwrap_or("img.jpg".to_string()));
    let content_type = filename.as_ref().map(|f| {
        new_mime_guess::from_path(f)
            .first_raw()
            .unwrap_or("image/jpeg")
            .to_owned()
    });

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

    let f = match data.picture {
        Some(f) => Some(tokio::fs::File::from_std(f.contents.as_file().try_clone().map_err(
            |e| AppError::InternalError(format!("could not clone file handle: {}", e)),
        )?)),
        _ => None,
    };

    let res = actions::create_picture(&values, f, &mut conn).await;

    if let Ok(picture) = res {
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
    } else {
        let error = res.unwrap_err().to_string();

        Ok(New {
            lang: "en",
            title: Some("New picture"),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            form_data: values,
            error: Some(error),
        }
        .into_response())
    }
}
