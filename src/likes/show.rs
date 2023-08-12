use askama::Template;
use axum::extract::Path;
use axum::extract::State;

use super::actions;
use crate::{errors::AppError, models::Like, uri_helpers::*, utils as filters, AppState, AuthContext};

#[derive(Template)]
#[template(path = "likes/show.html.jinja")]
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

pub async fn show(auth: AuthContext, State(state): State<AppState>, id: Path<i32>) -> Result<Show<'static>, AppError> {
    let mut conn = state.pool.acquire().await?;
    let like = actions::get_like(id.0, &mut conn).await?;

    Ok(Show {
        lang: "en",
        title: Some(format!("♥  {}", like.in_reply_to)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.current_user.is_some(),
        like,
        index: false,
        atom: false,
    })
}
