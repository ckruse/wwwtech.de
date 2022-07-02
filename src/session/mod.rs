use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;
use serde::{Deserialize, Serialize};

use crate::uri_helpers;
use crate::DbPool;

use crate::uri_helpers::*;

pub mod actions;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Template)]
#[template(path = "login.html.jinja")]
struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    email: &'a String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(new_session).service(login).service(logout);
}

#[get("/login")]
pub async fn new_session(id: Identity) -> Result<HttpResponse, Error> {
    let s = Show {
        lang: "en",
        title: Some("Login"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
        email: &"".to_owned(),
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/login")]
pub async fn login(id: Identity, form: web::Form<LoginForm>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let email = form.email.clone();
    let password = form.password.clone();

    let author = web::block(move || {
        let conn = pool.get()?;
        actions::get_author_by_email(&email, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    if actions::verify_password(&author, &password) {
        id.remember(author.id.to_string());
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, uri_helpers::root_uri()))
            .finish())
    } else {
        let s = Show {
            lang: "en",
            title: Some("Login"),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: false,
            email: &form.email,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body(s))
    }
}

#[post("/logout")]
pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, uri_helpers::root_uri()))
        .finish())
}
