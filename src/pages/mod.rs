use actix_identity::Identity;
use actix_web::{get, web, Error, HttpResponse, Result};
use askama::Template;
use chrono::Duration;

use crate::{caching_middleware, uri_helpers::*};

pub mod actions;
pub mod index;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(caching_middleware::Caching {
                duration: Duration::hours(1),
            })
            .service(index::index)
            .service(index::index_atom)
            .service(software)
            .service(about)
            .service(more),
    );
}

#[derive(Template)]
#[template(path = "pages/software.html.jinja")]
struct Software<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

#[get("/software")]
pub async fn software(id: Identity) -> Result<HttpResponse, Error> {
    let s = Software {
        lang: "en",
        title: Some("Software"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "pages/about.html.jinja")]
struct About<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

#[get("/about")]
pub async fn about(id: Identity) -> Result<HttpResponse, Error> {
    let s = About {
        lang: "en",
        title: Some("About me"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "pages/more.html.jinja")]
struct More<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

#[get("/more")]
pub async fn more(id: Identity) -> Result<HttpResponse, Error> {
    let s = More {
        lang: "en",
        title: Some("More…"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
