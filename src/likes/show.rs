use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use sqlx::PgConnection;

use super::actions;
use crate::errors::AppError;
use crate::models::Like;
use crate::uri_helpers::*;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "likes/show.html.j2")]
pub struct Show<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    like: Like,
    index: bool,
    atom: bool,
}

pub async fn show(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let like = get_like(id, &state, &mut conn).await?;

    let html = Show {
        lang: "en",
        title: Some(format!("♥  {}", like.in_reply_to)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
        like,
        index: false,
        atom: false,
    }
    .render()?;

    Ok(Html(html))
}

async fn get_like(id: i32, state: &AppState, conn: &mut PgConnection) -> Result<Like, AppError> {
    let like = match state.like_cache.get(&id).await {
        Some(like) => like,
        None => {
            let like = actions::get_like(id, conn).await?;
            state.like_cache.insert(id, like.clone()).await;

            like
        }
    };

    Ok(like)
}
