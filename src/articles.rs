use axum::{
    middleware::map_response_with_state,
    routing::{get, post},
    Router,
};
use chrono::Duration;

use crate::{middleware::caching_middleware, AppRouter, RequireAuth};

pub mod actions;
pub mod archive;
pub mod delete;
pub mod edit;
pub mod index;
pub mod new;
pub mod show;

static PER_PAGE: i64 = 10;

pub fn configure(app: AppRouter) -> AppRouter {
    let authed_router: AppRouter = Router::new()
        .route("/admin/articles/new", get(new::new))
        .route("/admin/articles", post(new::create))
        .route("/admin/articles/:id/edit", get(edit::edit))
        .route("/admin/articles/:id", post(edit::update))
        .route("/admin/articles/:id/delete", post(delete::delete))
        .route_layer(RequireAuth::login());

    let caching_router: AppRouter = Router::new()
        .route("/articles", get(index::index))
        .route("/articles/:year", get(archive::yearly_view))
        .route("/articles/:year/:month", get(archive::monthly_view))
        .route("/articles/:year/:month/:slug", get(show::show))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    app.route("/articles.atom", get(index::index_atom))
        .merge(authed_router)
        .merge(caching_router)
}
