use actix_files as fs;
// use actix_session::{CookieSession, Session};
// use actix_utils::mpsc;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use std::{env, io};
use tera::Tera;

pub mod uri_helpers;
pub mod utils;

pub mod pages;
pub mod static_handlers;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let base_path = utils::base_path();

    HttpServer::new(move || {
        let mut dir = base_path.clone();
        dir.push_str("/templates/**/*");

        let static_path = utils::static_path();
        let mut tera = Tera::new(dir.as_str()).unwrap();

        tera.register_function("asset_uri", uri_helpers::tera_asset_uri);
        tera.register_function("root_uri", uri_helpers::tera_root_uri);
        tera.register_function("page_uri", uri_helpers::tera_page_uri);

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .service(static_handlers::favicon)
            .service(static_handlers::gpgkey)
            .service(fs::Files::new("/static", static_path).show_files_listing())
            .service(pages::index)
            .service(pages::software)
            .service(pages::about)
            .service(pages::more)
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
