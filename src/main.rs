use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use axum::middleware::map_response_with_state;
use axum_login::AuthManagerLayerBuilder;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use moka::future::Cache;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
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

    pub article_cache: Cache<String, models::Article>,
    pub note_cache: Cache<i32, models::Note>,
    pub picture_cache: Cache<i32, models::Picture>,
    pub like_cache: Cache<i32, models::Like>,
    pub deafie_cache: Cache<String, models::Deafie>,
}
type AppRouter = Router<AppState>;
pub type AuthSession = axum_login::AuthSession<store::Store>;

static MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024;

#[cfg(debug_assertions)]
static SECURE: bool = false;
#[cfg(not(debug_assertions))]
static SECURE: bool = true;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(SECURE)
        .with_expiry(Expiry::OnInactivity(axum_login::tower_sessions::cookie::time::Duration::days(14)));

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/termitool_dev".to_owned());

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

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    posse::mastodon::verify_or_register()
        .await
        .expect("Error verifying Mastodon credentials");

    let static_path = utils::static_path();
    let serve_dir = ServeDir::new(static_path);

    let state = AppState {
        pool,
        article_cache: Cache::new(1000),
        note_cache: Cache::new(1000),
        picture_cache: Cache::new(1000),
        like_cache: Cache::new(1000),
        deafie_cache: Cache::new(1000),
    };

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
        .route_service("/A99A9D73.asc", ServeFile::new(utils::static_file_path("/A99A9D73.asc")))
        .route_service("/humans.txt", ServeFile::new(utils::static_file_path("humans.txt")))
        .route_service("/security.txt", ServeFile::new(utils::static_file_path("security.txt")))
        .route_service("/.well-known/security.txt", ServeFile::new(utils::static_file_path("security.txt")))
        .layer(map_response_with_state(chrono::Duration::days(365), middleware::caching_middleware));

    let app = app
        .merge(static_router)
        .with_state(state)
        .layer(AuthManagerLayerBuilder::new(user_store, session_layer).build())
        .layer(axum::middleware::map_response(middleware::webmention_middleware))
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
