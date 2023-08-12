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

static PER_PAGE: i64 = 50;

pub fn configure(app: AppRouter) -> AppRouter {
    let authed_router: AppRouter = Router::new()
        .route("/likes/new", get(new::new))
        .route("/likes", post(new::create))
        .route("/likes/:id/edit", get(edit::edit))
        .route("/likes/:id", post(edit::update))
        .route("/likes/:id/delete", post(delete::delete))
        .route_layer(RequireAuth::login());

    let caching_router: AppRouter = Router::new()
        .route("/likes", get(index::index))
        .route("/likes/:id", get(show::show))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    app.merge(authed_router)
        .merge(caching_router)
        .route("/likes.atom", get(index::index_atom))
}
