use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::DbPool;

use super::actions;
use crate::models::Like;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "likes/show.html.jinja")]
struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    like: &'a Like,
    index: bool,
    atom: bool,
}

#[get("/{id}")]
pub async fn show(ident: Option<Identity>, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let like = web::block(move || {
        let conn = pool.get()?;
        actions::get_like(id.into_inner(), &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Show {
        lang: "en",
        title: Some(&format!("♥ {}", like.in_reply_to)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: ident.is_some(),
        like: &like,
        index: false,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
