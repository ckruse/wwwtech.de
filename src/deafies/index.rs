use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::models::Deafie;
use crate::utils::paging::{get_page, get_paging, PageParams, Paging};
use crate::DbPool;

use super::{actions, PER_PAGE};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "deafies/index.html.jinja")]
struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    deafies: &'a Vec<Deafie>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
    // home: bool,
}

#[get("")]
pub async fn index(id: Identity, pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let logged_in = id.identity().is_some();
    let pool_ = pool.clone();
    let deafies = web::block(move || {
        let conn = pool_.get()?;
        actions::list_deafies(PER_PAGE, p * PER_PAGE, !logged_in, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_deafies(true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let s = Index {
        lang: "de",
        title: Some("Training a deaf dog: einen gehörlosen Hund ausbilden"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
        deafies: &deafies,
        paging: &paging,
        index: true,
        atom: false,
        // home: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
