use askama::Template;
use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect, Response},
};

use super::actions;
use crate::{
    errors::AppError,
    models::{Like, NewLike},
    uri_helpers::*,
    webmentions::send::send_mentions,
    AppState, AuthSession,
};

#[derive(Template)]
#[template(path = "likes/edit.html.jinja")]
pub struct Edit<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    like: Like,
    form_data: NewLike,
    error: Option<String>,
}

pub async fn edit(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Edit<'static>, AppError> {
    let mut conn = state.pool.acquire().await?;
    let like = actions::get_like(id, &mut conn).await?;

    Ok(Edit {
        lang: "en",
        title: Some(format!("Edit like #{}", like.id)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        form_data: NewLike {
            author_id: None,
            in_reply_to: like.in_reply_to.clone(),
            posse: like.posse,
            show_in_index: like.show_in_index,
            inserted_at: None,
            updated_at: None,
        },
        like,
        error: None,
    })
}

pub async fn update(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(mut form): Form<NewLike>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut conn = state.pool.acquire().await?;
    let like = actions::get_like(id, &mut conn).await?;

    form.author_id = Some(user.id);

    match actions::update_like(like.id, &form, &mut conn).await {
        Ok(like) => {
            let uri = like_uri(&like);

            tokio::task::spawn_blocking(move || {
                let uri = like_uri(&like);
                let _ = send_mentions(&uri);
            });

            Ok(Redirect::to(&uri).into_response())
        }

        Err(error) => Ok(Edit {
            lang: "en",
            title: Some(format!("Edit like #{}", like.id)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            like,
            form_data: form,
            error: Some(error.to_string()),
        }
        .into_response()),
    }
}
