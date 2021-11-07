use actix_files as fs;
// use actix_session::{CookieSession, Session};
// use actix_utils::mpsc;
use actix_web::http::StatusCode;
use actix_web::{get, guard, middleware, web, App, HttpResponse, HttpServer, Result};
use std::{env, io};
use tera::Tera;

pub mod uri_helpers;
pub mod utils;

pub mod pages;

#[get("/favicon.ico")]
async fn favicon() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/favicon.ico");

    Ok(fs::NamedFile::open(path)?)
}

#[get("/A99A9D73.asc")]
async fn gpgkey() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("/A99A9D73.asc");

    Ok(fs::NamedFile::open(path)?)
}

async fn p404() -> Result<fs::NamedFile> {
    let mut path = utils::static_path();
    path.push_str("404.html");

    Ok(fs::NamedFile::open(path)?.set_status_code(StatusCode::NOT_FOUND))
}

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
            .service(favicon)
            .service(gpgkey)
            .service(fs::Files::new("/static", static_path).show_files_listing())
            .service(pages::index)
            .service(pages::software)
            .service(pages::about)
            .service(pages::more)
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
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
