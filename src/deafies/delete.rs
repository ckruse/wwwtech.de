use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

use super::actions;
use crate::{errors::AppError, uri_helpers::*, AppState};

pub async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let deafie = actions::get_deafie(id, false, &mut conn).await?;

    actions::delete_deafie(deafie.id, &mut conn).await?;

    Ok(Redirect::to(&deafies_uri()))
}
