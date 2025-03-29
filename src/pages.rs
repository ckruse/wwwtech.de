use askama::Template;
use axum::Router;
use axum::middleware::map_response_with_state;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use chrono::Duration;

use crate::errors::AppError;
use crate::middleware::caching_middleware;
use crate::uri_helpers::*;
use crate::{AppRouter, AuthSession};

pub mod actions;
pub mod index;

pub fn configure(router: AppRouter) -> AppRouter {
    let caching_router: AppRouter = Router::new()
        .route("/", get(index::index))
        .route("/whatsnew.atom", get(index::index_atom))
        .route("/software", get(software))
        .route("/about", get(about))
        .route("/more", get(more))
        .layer(map_response_with_state(Duration::hours(1), caching_middleware));

    router.merge(caching_router)
}

#[derive(Template)]
#[template(path = "pages/software.html.j2")]
pub struct Software<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

pub async fn software(auth: AuthSession) -> Result<impl IntoResponse, AppError> {
    let html = Software {
        lang: "en",
        title: Some("Software"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "pages/about.html.j2")]
pub struct About<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

pub async fn about(auth: AuthSession) -> Result<impl IntoResponse, AppError> {
    let html = About {
        lang: "en",
        title: Some("About me"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "pages/more.html.j2")]
pub struct More<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

pub async fn more(auth: AuthSession) -> Result<impl IntoResponse, AppError> {
    let html = More {
        lang: "en",
        title: Some("More…"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),
    }
    .render()?;

    Ok(Html(html))
}
