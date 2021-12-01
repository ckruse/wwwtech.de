use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

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
    logged_in: bool,

    articles: &'a Vec<Article>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
}

#[get("/articles")]
pub async fn index(id: Identity, pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<HttpResponse, Error> {
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
        logged_in: id.identity().is_some(),
        articles: &articles,
        paging: &paging,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "articles/article.html.jinja")]
pub struct ArticleTpl<'a> {
    pub article: &'a Article,
    pub index: bool,
    pub atom: bool,
}

#[get("/articles.atom")]
pub async fn index_atom(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let articles = web::block(move || {
        let conn = pool_.get()?;
        actions::list_articles(50, 0, true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let newest_article = articles.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at)).unwrap();
    let updated_at: DateTime<Utc> = DateTime::from_utc(newest_article.updated_at, Utc);

    let entries: Vec<Entry> = articles
        .iter()
        .map(|article| {
            let fixed_tz = Local.offset_from_utc_datetime(&article.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&article.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_utc(article.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Article/{}", article.id))
                .published(inserted)
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(article_uri(article))
                        .mime_type("text/html".to_string())
                        .rel("alternate")
                        .build(),
                )
                .title(article.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type("html".to_string())
                        .value(
                            ArticleTpl {
                                article: &article,
                                index: false,
                                atom: true,
                            }
                            .render()
                            .unwrap(),
                        )
                        .build(),
                )
                .build()
        })
        .collect();

    let s = FeedBuilder::default()
        .lang("en-US".to_string())
        .id(articles_atom_uri())
        .title("WWWTech / Articles")
        .link(
            LinkBuilder::default()
                .href(articles_uri())
                .mime_type("text/html".to_string())
                .rel("alternate")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(articles_atom_uri())
                .mime_type("application/atom+xml".to_string())
                .rel("self")
                .build(),
        )
        .updated(updated_at)
        .author(
            PersonBuilder::default()
                .name("Christian Kruse")
                .email("christian@kruse.cool".to_string())
                .uri("https://wwwtech.de/about".to_string())
                .build(),
        )
        .entries(entries)
        .build()
        .to_string();

    Ok(HttpResponse::Ok()
        .content_type("application/atom+xml; charset=utf-8")
        .body(s))
}