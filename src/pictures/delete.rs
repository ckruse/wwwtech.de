use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

use super::actions;
use crate::{errors::AppError, uri_helpers::*, AppState};

pub async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let picture = actions::get_picture(id, &mut conn).await?;

    let _deleted = actions::delete_picture(&picture, &mut conn).await?;

    Ok(Redirect::to(&pictures_uri()))
}
