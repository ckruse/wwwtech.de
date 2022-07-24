#[macro_use]
extern crate diesel_migrations;
#[cfg(debug_assertions)]
extern crate dotenv;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate anyhow;

use actix_files as fs;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use chrono::Duration;
use std::{env, io};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[cfg(debug_assertions)]
use dotenv::dotenv;

use uri_helpers::webmentions_endpoint_uri;
pub mod caching_middleware;
pub mod multipart;
pub mod uri_helpers;
pub mod utils;

pub mod models;
pub mod schema;

pub mod api;
pub mod articles;
pub mod deafies;
pub mod likes;
pub mod notes;
pub mod pages;
pub mod pictures;
pub mod session;
pub mod static_handlers;
pub mod webmentions;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

embed_migrations!("./migrations/");

#[actix_web::main]
async fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    #[cfg(not(debug_assertions))]
    let _guard = sentry::init((
        env::var("SENTRY_ENDPOINT").expect("SENTRY_ENDPOINT"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    let master_key = env::var("COOKIE_KEY").expect("env variable COOKIE_KEY not set");
    let secret_key = Key::derive_from(master_key.as_bytes());
    env_logger::init();

    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let db_conn = pool.get().expect("could not get connection");
    embedded_migrations::run(&db_conn).expect("could not run migrations!");

    HttpServer::new(move || {
        let static_path = utils::static_path();
        let json_cfg = web::JsonConfig::default().limit(20971520);

        App::new()
            .wrap(sentry_actix::Sentry::new())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(json_cfg))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("link", format!("<{}>; rel=\"webmention\"", webmentions_endpoint_uri()))),
            )
            .configure(api::routes)
            .service(static_handlers::favicon)
            .service(static_handlers::robots_txt)
            .service(static_handlers::gpgkey)
            .service(static_handlers::humans_txt)
            .service(static_handlers::well_known_security_txt)
            .service(static_handlers::security_txt)
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
            .configure(deafies::routes)
            .configure(notes::routes)
            .configure(pictures::routes)
            .configure(likes::routes)
            .configure(webmentions::routes)
            .configure(pages::routes)
            .default_service(web::to(static_handlers::p404))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
