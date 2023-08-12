use askama::Template;
use axum::{
    extract::{Extension, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_typed_multipart::TypedMultipart;

use super::{actions, DeafieData};
use crate::{
    errors::AppError,
    models::{generate_deafie_pictures, Author, NewDeafie},
    posse::mastodon::post_deafie,
    uri_helpers::*,
    utils as filters,
    webmentions::send::send_mentions,
    AppState,
};

#[derive(Template)]
#[template(path = "deafies/new.html.jinja")]
pub(crate) struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: NewDeafie,
    error: Option<String>,
}

pub(crate) async fn new() -> New<'static> {
    New {
        lang: "de",
        title: Some("New deafie article"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: None,
        form_data: NewDeafie { ..Default::default() },
    }
}

pub(crate) async fn create(
    Extension(user): Extension<Author>,
    State(state): State<AppState>,
    TypedMultipart(data): TypedMultipart<DeafieData>,
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

    let values = NewDeafie {
        title: data.title,
        slug: data.slug,
        image_name: filename,
        image_content_type: content_type,
        excerpt: data.excerpt,
        body: data.body,
        published: data.published,
        posse_visibility: data.posse_visibility,
        content_warning: data.content_warning,
        author_id: Some(user.id),
        ..Default::default()
    };

    let f = match data.picture {
        Some(f) => Some(tokio::fs::File::from_std(f.contents.as_file().try_clone().map_err(
            |e| AppError::InternalError(format!("could not clone file handle: {}", e)),
        )?)),
        _ => None,
    };

    let res = actions::create_deafie(&values, f, &mut conn).await;

    if let Ok(deafie) = res {
        let uri = deafie_uri(&deafie);

        tokio::task::spawn_blocking(move || {
            let _ = generate_deafie_pictures(&deafie);

            if deafie.published {
                let uri = deafie_uri(&deafie);
                let _ = send_mentions(&uri);

                tokio::task::spawn(async move {
                    let _ = post_deafie(&deafie).await;
                });
            }
        });

        Ok(Redirect::to(&uri).into_response())
    } else {
        let error = res.unwrap_err().to_string();

        Ok(New {
            lang: "de",
            title: Some("New deafie article"),
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
