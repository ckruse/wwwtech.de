#[cfg(debug_assertions)]
extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate argonautica;
#[macro_use]
extern crate anyhow;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::rt::Arbiter;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use background_jobs::memory_storage::Storage;
use background_jobs::{create_server, WorkerConfig};
use chrono::Duration;
use std::{env, io};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[cfg(debug_assertions)]
use dotenv::dotenv;

use models::Picture;
use uri_helpers::webmentions_endpoint_uri;
use webmentions::send::WebmenentionSenderJob;
pub mod caching_middleware;
pub mod multipart;
pub mod uri_helpers;
pub mod utils;

pub mod models;
pub mod schema;

pub mod articles;
pub mod likes;
pub mod notes;
pub mod pages;
pub mod pictures;
pub mod session;
pub mod static_handlers;
pub mod webmentions;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

const DEFAULT_QUEUE: &str = "default";
const ASSET_VERSION: &str = "a";

#[actix_web::main]
async fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        let static_path = utils::static_path();

        let storage = Storage::new();
        let queue = create_server(storage);

        WorkerConfig::new(|| ())
            .register::<WebmenentionSenderJob>()
            .register::<Picture>()
            .set_worker_count(DEFAULT_QUEUE, 1)
            .start_in_arbiter(&Arbiter::default(), queue.clone());

        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32]).name("wwwtech").secure(false),
            ))
            .data(pool.clone())
            .data(queue.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("link", format!("<{}>; rel=\"webmention\"", webmentions_endpoint_uri())),
            )
            .service(static_handlers::favicon)
            .service(static_handlers::robots_txt)
            .service(static_handlers::gpgkey)
            .service(static_handlers::humans_txt)
            .service(static_handlers::keybase_txt)
            .service(
                web::scope("/static")
                    .wrap(caching_middleware::Caching {
                        duration: Duration::days(365),
                    })
                    .service(
                        fs::Files::new("", static_path)
                            .show_files_listing()
                            .use_last_modified(true),
                    ),
            )
            .configure(session::routes)
            .configure(articles::routes)
            .configure(notes::routes)
            .configure(pictures::routes)
            .configure(likes::routes)
            .configure(webmentions::routes)
            .configure(pages::routes)
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(static_handlers::p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
