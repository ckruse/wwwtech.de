use axum::{
    middleware::map_response_with_state,
    routing::{get, post},
    Router,
};
use chrono::Duration;

use crate::{middleware::caching_middleware, AppRouter, RequireAuth};

pub mod actions;

pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 25;

pub fn configure(app: AppRouter) -> AppRouter {
    let authed_router: AppRouter = Router::new()
        .route("/notes/new", get(new::new))
        .route("/notes", post(new::create))
        .route("/notes/:id/edit", get(edit::edit))
        .route("/notes/:id", post(edit::update))
        .route("/notes/:id/delete", post(delete::delete))
        .route_layer(RequireAuth::login());

    let caching_router: AppRouter = Router::new()
        .route("/notes", get(index::index))
        .route("/notes/:id", get(show::show))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    app.route("/notes.atom", get(index::index_atom))
        .merge(authed_router)
        .merge(caching_router)
}