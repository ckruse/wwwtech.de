use axum::Router;
use axum::middleware::map_response_with_state;
use axum::routing::{get, post};
use axum_login::login_required;
use chrono::Duration;

use crate::AppRouter;
use crate::middleware::caching_middleware;
use crate::store::Store;

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
        .route_layer(login_required!(Store, login_url = "/login"));

    let caching_router: AppRouter = Router::new()
        .route("/likes", get(index::index))
        .route("/likes/:id", get(show::show))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    app.merge(authed_router)
        .merge(caching_router)
        .route("/likes.atom", get(index::index_atom))
}
