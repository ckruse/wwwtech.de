use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};

use super::actions;
use crate::AppState;
use crate::errors::AppError;
use crate::uri_helpers::*;

pub async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let picture = actions::get_picture(id, &mut conn).await?;

    actions::delete_picture(&picture, &mut conn).await?;
    state.picture_cache.remove(&picture.id).await;

    Ok(Redirect::to(&pictures_uri()))
}
