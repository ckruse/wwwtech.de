use axum::{
    middleware::map_response_with_state,
    routing::{get, post},
    Router,
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::Duration;
use tempfile::NamedTempFile;

use crate::{middleware::caching_middleware, AppRouter, RequireAuth};

pub mod actions;
pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 10;

#[derive(TryFromMultipart)]
pub struct DeafieData {
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub body: String,
    #[form_data(default)]
    pub published: bool,
    pub posse_visibility: String,
    pub content_warning: Option<String>,

    #[form_data(limit = "unlimited")]
    pub picture: Option<FieldData<NamedTempFile>>,
}

pub fn configure(app: AppRouter) -> AppRouter {
    let authed_router: AppRouter = Router::new()
        .route("/admin/the-life-of-alfons/new", get(new::new))
        .route("/admin/the-life-of-alfons", post(new::create))
        .route("/admin/the-life-of-alfons/:id/edit", get(edit::edit))
        .route("/admin/the-life-of-alfons/:id", post(edit::update))
        .route("/admin/the-life-of-alfons/:id/delete", post(delete::delete))
        .route_layer(RequireAuth::login());

    let caching_router: AppRouter = Router::new()
        .route("/the-life-of-alfons", get(index::index))
        .route("/the-life-of-alfons/:year/:month/:slug", get(show::show))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    app.merge(authed_router)
        .merge(caching_router)
        .route("/the-life-of-alfons.atom", get(index::index_atom))
}
