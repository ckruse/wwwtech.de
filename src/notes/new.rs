use askama::Template;
use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};

use super::actions;
use crate::errors::AppError;
use crate::models::NewNote;
use crate::posse::mastodon::post_note;
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "notes/new.html.j2")]
pub struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: NewNote,
    error: Option<String>,
}

pub async fn new() -> Result<impl IntoResponse, AppError> {
    let html = New {
        lang: "en",
        title: Some("New note"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: None,
        form_data: NewNote {
            note_type: "note".to_owned(),
            lang: "en".to_owned(),
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
    Form(form): Form<NewNote>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut data = form.clone();
    data.author_id = Some(user.id);
    let mut conn = state.pool.acquire().await?;

    match actions::create_note(&data, &mut conn).await {
        Ok(note) => {
            let uri = note_uri(&note);

            if note.posse {
                let note_ = note.clone();
                tokio::task::spawn(async move {
                    let _ = post_note(&note_).await;
                });
            }

            tokio::task::spawn_blocking(move || {
                let uri = note_uri(&note);
                let _ = send_mentions(&uri);
            });

            Ok(Redirect::to(&uri).into_response())
        }

        Err(error) => {
            let html = New {
                lang: "en",
                title: Some("New note"),
                page_type: None,
                page_image: None,
                body_id: None,
                logged_in: true,
                form_data: form,
                error: Some(error.to_string()),
            }
            .render()?;

            Ok((StatusCode::UNAUTHORIZED, Html(html)).into_response())
        }
    }
}
