use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use sqlx::PgConnection;

use super::actions;
use crate::errors::AppError;
use crate::models::Note;
use crate::uri_helpers::*;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "notes/show.html.j2")]
pub struct Show<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    note: Note,
    index: bool,
    atom: bool,
}

pub async fn show(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let note = get_note(id, &state, &mut conn).await?;

    let html = Show {
        lang: "en",
        title: Some(note.title.clone()),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
        note,
        index: false,
        atom: false,
    }
    .render()?;

    Ok(Html(html))
}

async fn get_note(id: i32, state: &AppState, conn: &mut PgConnection) -> Result<Note, AppError> {
    let note = match state.note_cache.get(&id).await {
        Some(note) => note,
        None => {
            let note = actions::get_note(id, conn).await?;
            state.note_cache.insert(id, note.clone()).await;

            note
        }
    };

    Ok(note)
}
