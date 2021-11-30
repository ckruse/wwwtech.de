use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::models::Article;
use crate::DbPool;

use crate::articles::actions as article_actions;

use self::actions::NotePictureLike;

use crate::uri_helpers::*;
use crate::utils as filters;

pub mod actions;

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
    logged_in: bool,

    home: bool,
    index: bool,
    atom: bool,
    picture_type: &'a str,

    article: &'a Article,
    items: &'a Vec<Vec<NotePictureLike>>,
}

#[get("/")]
pub async fn index(id: Identity, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let article = web::block(move || {
        let conn = pool_.get()?;
        article_actions::get_youngest_article(true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let items = actions::get_last_ten_items(&pool).await?;

    let grouped_items: Vec<Vec<NotePictureLike>> = {
        let mut groups = Vec::new();
        let mut this_group: Vec<NotePictureLike> = Vec::new();

        for item in items {
            if this_group.is_empty()
                || actions::inserted_at_for(&this_group[0]).date() == actions::inserted_at_for(&item).date()
            {
                this_group.push(item);
            } else {
                groups.push(this_group);
                this_group = vec![item];
            }
        }
        groups
    };

    let s = Index {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),

        home: true,
        index: true,
        atom: false,
        picture_type: "thumbnail",

        article: &article,
        items: &grouped_items,
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
    logged_in: bool,
}

#[get("/software")]
pub async fn software(id: Identity) -> Result<HttpResponse, Error> {
    let s = Software {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
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
    logged_in: bool,
}

#[get("/about")]
pub async fn about(id: Identity) -> Result<HttpResponse, Error> {
    let s = About {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
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
    logged_in: bool,
}

#[get("/more")]
pub async fn more(id: Identity) -> Result<HttpResponse, Error> {
    let s = More {
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
