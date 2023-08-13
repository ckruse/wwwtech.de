use std::{net::SocketAddr, time::Duration};

use axum::{middleware::map_response_with_state, Router};
#[cfg(debug_assertions)]
use axum_login::axum_sessions::async_session::CookieStore as SessionStore;
#[cfg(not(debug_assertions))]
use axum_login::axum_sessions::async_session::MemoryStore as SessionStore;
use axum_login::{
    axum_sessions::{PersistencePolicy, SessionLayer},
    AuthLayer, RequireAuthorizationLayer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::services::{ServeDir, ServeFile};

mod articles;
mod deafies;
mod errors;
mod likes;
mod middleware;
mod models;
mod notes;
mod pages;
mod pictures;
mod posse;
mod session;
mod store;
mod uri_helpers;
mod utils;
mod webmentions;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
}
type AppRouter = Router<AppState>;
type AuthContext = axum_login::extractors::AuthContext<i32, models::Author, store::Store>;
type RequireAuth = RequireAuthorizationLayer<i32, models::Author>;

static MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let secret = std::env::var("COOKIE_KEY").expect("COOKIE_KEY not set!");

    let session_store = SessionStore::new();
    let session_layer =
        SessionLayer::new(session_store, secret.as_bytes()).with_persistence_policy(PersistencePolicy::ExistingOnly);

    let database_url = std::env::var("DATABASE_URL").unwrap_or("postgres://localhost/termitool_dev".to_string());

    let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
        .as_deref()
        .unwrap_or("5")
        .parse::<u32>()
        .expect("DATABASE_MAX_CONNECTIONS must be a number");

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(3)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let user_store = store::Store::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, secret.as_bytes());

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    posse::mastodon::verify_or_register()
        .await
        .expect("Error verifying Mastodon credentials");

    let static_path = utils::static_path();
    let serve_dir = ServeDir::new(static_path);

    let state = AppState { pool };
    let mut app: AppRouter = Router::new();
    app = pages::configure(app);
    app = articles::configure(app);
    app = notes::configure(app);
    app = likes::configure(app);
    app = pictures::configure(app);
    app = deafies::configure(app);
    app = session::configure(app);
    app = webmentions::configure(app);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    tracing::debug!("listening on {}", addr);

    let static_router: AppRouter = Router::new()
        .nest_service("/static", serve_dir)
        .route_service("/favicon.ico", ServeFile::new(utils::static_file_path("favicon.ico")))
        .route_service("/robots.txt", ServeFile::new(utils::static_file_path("robots.txt")))
        .route_service(
            "/A99A9D73.asc",
            ServeFile::new(utils::static_file_path("/A99A9D73.asc")),
        )
        .route_service("/humans.txt", ServeFile::new(utils::static_file_path("humans.txt")))
        .route_service("/security.txt", ServeFile::new(utils::static_file_path("security.txt")))
        .route_service(
            "/.well-known/security.txt",
            ServeFile::new(utils::static_file_path("security.txt")),
        )
        .layer(map_response_with_state(
            chrono::Duration::days(365),
            middleware::caching_middleware,
        ));

    let app = app
        .merge(static_router)
        .with_state(state)
        .layer(auth_layer)
        .layer(session_layer)
        .layer(axum::middleware::map_response(middleware::webmention_middleware))
        .into_make_service();

    axum::Server::bind(&addr).serve(app).await.unwrap();
}
