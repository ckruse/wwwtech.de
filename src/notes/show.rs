use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::DbPool;

use super::actions;
use crate::models::Note;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "notes/show.html.jinja")]
struct Show<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    note: &'a Note,
    index: bool,
    atom: bool,
}

#[get("/{id}")]
pub async fn show(ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let note = web::block(move || {
        let conn = pool.get()?;
        actions::get_note(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Show {
        title: Some(&note.title),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in: ident.identity().is_some(),
        note: &note,
        index: false,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
