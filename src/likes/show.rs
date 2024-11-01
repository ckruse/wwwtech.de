use askama::Template;
use axum::extract::{Path, State};

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

pub async fn show(auth: AuthSession, State(state): State<AppState>, id: Path<i32>) -> Result<Show<'static>, AppError> {
    let mut conn = state.pool.acquire().await?;
    let like = actions::get_like(id.0, &mut conn).await?;

    Ok(Show {
        lang: "en",
        title: Some(format!("♥  {}", like.in_reply_to)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
        like,
        index: false,
        atom: false,
    })
}
