extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate argonautica;
#[macro_use]
extern crate anyhow;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
// use actix_session::{CookieSession, Session};
// use actix_utils::mpsc;
use actix_web::rt::Arbiter;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use background_jobs::memory_storage::Storage;
use background_jobs::{create_server, WorkerConfig};
use std::{env, io};
use webmentions::send::WebmenentionSenderJob;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

use uri_helpers::webmentions_endpoint_uri;

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

#[actix_web::main]
async fn main() -> io::Result<()> {
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
            .set_worker_count(DEFAULT_QUEUE, 1)
            .start_in_arbiter(&Arbiter::default(), queue.clone());

        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32]).name("wwwtech").secure(false),
            ))
            .data(pool.clone())
            .data(queue.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("link", format!("<{}>; rel=\"webmention\"", webmentions_endpoint_uri())),
            )
            .service(static_handlers::favicon)
            .service(static_handlers::robots_txt)
            .service(static_handlers::gpgkey)
            .service(static_handlers::humans_txt)
            .service(fs::Files::new("/static", static_path).show_files_listing())
            .configure(session::routes)
            .configure(pages::routes)
            .configure(articles::routes)
            .configure(notes::routes)
            .configure(pictures::routes)
            .configure(likes::routes)
            .configure(webmentions::routes)
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
