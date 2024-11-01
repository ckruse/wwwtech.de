use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};

use super::actions;
use crate::AppState;
use crate::errors::AppError;
use crate::uri_helpers::*;

pub async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let article = actions::get_article(id, false, &mut conn).await?;

    actions::delete_article(article.id, &mut conn).await?;

    Ok(Redirect::to(&articles_uri()))
}
