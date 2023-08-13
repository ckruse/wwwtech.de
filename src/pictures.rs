use axum::{
    middleware::map_response_with_state,
    routing::{get, post},
    Router,
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use crate::{middleware::caching_middleware, AppRouter, RequireAuth};

pub mod actions;

pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 60;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ImageTypes {
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "original")]
    Original,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TypeParams {
    #[serde(rename = "type")]
    pub pic_type: Option<ImageTypes>,
}

#[derive(TryFromMultipart)]
pub struct PictureData {
    pub title: String,
    pub alt: Option<String>,
    pub content_warning: Option<String>,
    pub in_reply_to: Option<String>,
    pub lang: String,

    #[form_data(default)]
    pub posse: bool,
    pub posse_visibility: String,

    #[form_data(default)]
    pub show_in_index: bool,

    #[form_data(limit = "unlimited")]
    pub picture: FieldData<NamedTempFile>,

    pub content: Option<String>,
}

pub fn configure(app: AppRouter) -> AppRouter {
    let authed_router: AppRouter = Router::new()
        .route("/pictures/new", get(new::new))
        .route("/pictures", post(new::create))
        .route("/pictures/:id/edit", get(edit::edit))
        .route("/pictures/:id", post(edit::update))
        .route("/pictures/:id/delete", post(delete::delete))
        .route_layer(RequireAuth::login());

    let caching_router: AppRouter = Router::new()
        .route("/pictures", get(index::index))
        .route("/pictures/:id", get(show::show))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    app.merge(authed_router)
        .merge(caching_router)
        .route("/pictures.atom", get(index::index_atom))
}
