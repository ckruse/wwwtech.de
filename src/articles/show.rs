use actix_identity::Identity;
use actix_web::{error, get, http::header, web, Error, HttpResponse, Result};
use askama::Template;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::result::Error::NotFound;
use diesel::PgConnection;

use crate::DbPool;

use super::actions;
use crate::models::Article;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "articles/show.html.jinja")]
struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    article: &'a Article,
    index: bool,
    atom: bool,
}

async fn redirect_or_error(
    slug: String,
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    logged_in: bool,
) -> Result<HttpResponse, Error> {
    let article = web::block(move || actions::get_article_by_slug_part(&slug, !logged_in, &conn)).await?;

    match article {
        Ok(article) => Ok(HttpResponse::Found()
            .append_header((header::LOCATION, article_uri(&article)))
            .finish()),
        _ => Err(error::ErrorNotFound(format!("article could not be found"))),
    }
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
    let conn = pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let article = web::block(move || {
        let conn = pool.get()?;
        actions::get_article_by_slug(&guid, !logged_in, &conn)
    })
    .await?;

    let article = match article {
        Ok(article) => article,
        Err(e) => match e.downcast_ref::<diesel::result::Error>() {
            Some(NotFound) => return redirect_or_error(slug, conn, logged_in).await,
            _ => return Err(error::ErrorInternalServerError(format!("Database error: {}", e))),
        },
    };

    let s = Show {
        lang: "en",
        title: Some(&article.title.clone()),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in,
        article: &article,
        index: false,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
