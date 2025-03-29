use askama::Template;
use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};

use super::actions;
use crate::errors::AppError;
use crate::models::NewLike;
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession};

#[derive(Template)]
#[template(path = "likes/new.html.j2")]
pub struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: NewLike,
    error: Option<String>,
}

pub async fn new() -> Result<impl IntoResponse, AppError> {
    let html = New {
        lang: "en",
        title: Some("New like"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: None,
        form_data: NewLike {
            posse: true,
            show_in_index: true,
            ..Default::default()
        },
    }
    .render()?;

    Ok(Html(html))
}

pub async fn create(
    auth: AuthSession,
    State(state): State<AppState>,
    Form(mut form): Form<NewLike>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut conn = state.pool.acquire().await?;
    form.author_id = Some(user.id);
    let res = actions::create_like(&form, &mut conn).await;

    if let Ok(like) = res {
        let uri = like_uri(&like);

        tokio::task::spawn_blocking(move || {
            let uri = like_uri(&like);
            let _ = send_mentions(&uri);
        });

        Ok(Redirect::to(&uri).into_response())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let html = New {
            lang: "en",
            title: Some("New like"),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            form_data: form,
            error,
        }
        .render()?;

        Ok((StatusCode::UNPROCESSABLE_ENTITY, Html(html)).into_response())
    }
}
