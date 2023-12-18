use askama::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;

use super::actions;
use crate::{errors::AppError, models::Note, uri_helpers::*, utils as filters, AppState, AuthSession};

#[derive(Template)]
#[template(path = "notes/show.html.jinja")]
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
    id: Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let note = actions::get_note(id.0, &mut conn).await?;

    Ok(Show {
        lang: "en",
        title: Some(note.title.clone()),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
        note,
        index: false,
        atom: false,
    })
}
