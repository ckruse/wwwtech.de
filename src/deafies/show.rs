use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::DbPool;

use super::actions;
use crate::models::Deafie;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "deafies/show.html.jinja")]
struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    deafie: &'a Deafie,
    index: bool,
    atom: bool,
}

#[get("/{year}/{month}/{slug}")]
pub async fn show(
    ident: Identity,
    pool: web::Data<DbPool>,
    path: web::Path<(i32, String, String)>,
) -> Result<HttpResponse, Error> {
    let logged_in = ident.identity().is_some();
    let (year, month, slug) = path.into_inner();
    let guid = format!("{}/{}/{}", year, month, slug);

    let deafie = web::block(move || {
        let conn = pool.get()?;
        actions::get_deafie_by_slug(&guid, !logged_in, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Show {
        lang: "de",
        title: Some(&deafie.title.clone()),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in,
        deafie: &deafie,
        index: false,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
