use askama::Template;
use axum::{middleware::map_response_with_state, routing::get, Router};
use chrono::Duration;

use crate::{middleware::caching_middleware, uri_helpers::*, AppRouter, AuthContext};

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
#[template(path = "pages/software.html.jinja")]
pub struct Software<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

pub async fn software(auth: AuthContext) -> Software<'static> {
    Software {
        lang: "en",
        title: Some("Software"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.current_user.is_some(),
    }
}

#[derive(Template)]
#[template(path = "pages/about.html.jinja")]
pub struct About<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

pub async fn about(auth: AuthContext) -> About<'static> {
    About {
        lang: "en",
        title: Some("About me"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.current_user.is_some(),
    }
}

#[derive(Template)]
#[template(path = "pages/more.html.jinja")]
pub struct More<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
}

pub async fn more(auth: AuthContext) -> More<'static> {
    More {
        lang: "en",
        title: Some("More…"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.current_user.is_some(),
    }
}
