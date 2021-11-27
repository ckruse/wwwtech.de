use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::models::Article;
use crate::utils::paging::{get_page, get_paging, PageParams, Paging};
use crate::DbPool;

use super::{actions, PER_PAGE};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "articles/index.html.jinja")]
struct Index<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,

    articles: &'a Vec<Article>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
}

#[get("/articles")]
pub async fn index(pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let pool_ = pool.clone();
    let articles = web::block(move || {
        let conn = pool_.get()?;
        actions::list_articles(PER_PAGE, p * PER_PAGE, true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_articles(true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let s = Index {
        title: Some("Articles"),
        page_type: None,
        page_image: None,
        body_id: None,
        articles: &articles,
        paging: &paging,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
