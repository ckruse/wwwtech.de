use askama::Template;
use axum::extract::{Form, Path, State};
use axum::response::{IntoResponse, Redirect, Response};

use super::actions;
use crate::errors::AppError;
use crate::models::{NewNote, Note};
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "notes/edit.html.j2")]
pub struct Edit<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    note: Note,
    form_data: NewNote,
    error: Option<String>,
}

pub async fn edit(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Edit<'static>, AppError> {
    let mut conn = state.pool.acquire().await?;
    let note = actions::get_note(id, &mut conn).await?;

    Ok(Edit {
        lang: "en",
        title: Some(format!("Edit note #{}: {}", note.id, note.title)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        form_data: NewNote {
            author_id: None,
            title: note.title.clone(),
            note_type: note.note_type.clone(),
            in_reply_to: note.in_reply_to.clone(),
            lang: note.lang.clone(),
            posse: note.posse,
            show_in_index: note.show_in_index,
            content: Some(note.content.clone()),
            inserted_at: None,
            updated_at: None,
            posse_visibility: note.posse_visibility.clone(),
            content_warning: note.content_warning.clone(),
        },

        note,
        error: None,
    })
}

pub async fn update(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<NewNote>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut conn = state.pool.acquire().await?;
    let note = actions::get_note(id, &mut conn).await?;

    let mut data = form.clone();
    data.author_id = Some(user.id);

    match actions::update_note(note.id, &data, &mut conn).await {
        Ok(note) => {
            let uri = note_uri(&note);

            tokio::task::spawn_blocking(move || {
                let uri = note_uri(&note);
                let _ = send_mentions(&uri);
            });

            Ok(Redirect::to(&uri).into_response())
        }

        Err(error) => Ok(Edit {
            lang: "en",
            title: Some("Edit note".to_owned()),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            note,
            form_data: form,
            error: Some(error.to_string()),
        }
        .into_response()),
    }
}
