use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_typed_multipart::TypedMultipart;

use super::{DeafieData, actions};
use crate::errors::AppError;
use crate::models::{Deafie, NewDeafie, generate_deafie_pictures};
use crate::posse::mastodon::post_deafie;
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "deafies/edit.html.j2")]
pub(crate) struct Edit<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    deafie: Deafie,
    form_data: NewDeafie,
    error: Option<String>,
}

pub(crate) async fn edit(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let deafie = actions::get_deafie(id, false, &mut conn).await?;

    let html = Edit {
        lang: "de",
        title: Some(format!("Edit deafie „{}“", deafie.title)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        form_data: NewDeafie {
            title: deafie.title.clone(),
            slug: deafie.slug.clone(),
            excerpt: deafie.excerpt.clone(),
            body: deafie.body.clone(),
            published: deafie.published,
            posse_visibility: deafie.posse_visibility.clone(),
            content_warning: deafie.content_warning.clone(),
            ..Default::default()
        },

        deafie,
        error: None,
    }
    .render()?;

    Ok(Html(html))
}

pub async fn update(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    TypedMultipart(data): TypedMultipart<DeafieData>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

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

    let old_deafie = actions::get_deafie(id, false, &mut conn).await?;

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
        Some(f) => Some(tokio::fs::File::from_std(
            f.contents
                .as_file()
                .try_clone()
                .map_err(|e| AppError::InternalError(format!("could not clone file handle: {}", e)))?,
        )),
        _ => None,
    };

    match actions::update_deafie(old_deafie.id, &values, f, &mut conn).await {
        Ok(deafie) => {
            state.deafie_cache.insert(deafie.slug.clone(), deafie.clone()).await;

            let uri = deafie_uri(&deafie);

            tokio::task::spawn_blocking(move || {
                let _ = generate_deafie_pictures(&deafie);

                if deafie.published {
                    let uri = deafie_uri(&deafie);
                    let _ = send_mentions(&uri);

                    if !old_deafie.published {
                        tokio::task::spawn(async move {
                            let _ = post_deafie(&deafie).await;
                        });
                    }
                }
            });

            Ok(Redirect::to(&uri).into_response())
        }

        Err(error) => {
            let html = Edit {
                lang: "de",
                title: Some(format!("Edit deafie „{}“", old_deafie.title)),
                page_type: None,
                page_image: None,
                body_id: None,
                logged_in: true,
                deafie: old_deafie,
                form_data: values,
                error: Some(error.to_string()),
            }
            .render()?;

            Ok((StatusCode::UNPROCESSABLE_ENTITY, Html(html)).into_response())
        }
    }
}
