use actix_web::{get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::uri_helpers::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(software).service(about).service(more);
}

#[derive(Template)]
#[template(path = "pages/index.html.jinja")]
struct Index<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
}

#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    let s = Index {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "pages/software.html.jinja")]
struct Software<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
}

#[get("/software")]
pub async fn software() -> Result<HttpResponse, Error> {
    let s = Software {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "pages/about.html.jinja")]
struct About<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
}

#[get("/about")]
pub async fn about() -> Result<HttpResponse, Error> {
    let s = About {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "pages/more.html.jinja")]
struct More<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
}

#[get("/more")]
pub async fn more() -> Result<HttpResponse, Error> {
    let s = More {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
