use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use crate::models::Like;
use crate::utils::paging::{get_page, get_paging, PageParams, Paging};
use crate::DbPool;

use super::{actions, PER_PAGE};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "likes/index.html.jinja")]
struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    likes: &'a Vec<Like>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
}

#[get("")]
pub async fn index(id: Identity, pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let logged_in = id.identity().is_some();
    let pool_ = pool.clone();
    let likes = web::block(move || {
        let conn = pool_.get()?;
        actions::list_likes(PER_PAGE, p * PER_PAGE, !logged_in, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_likes(true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let s = Index {
        lang: "en",
        title: Some("Likes"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in,
        likes: &likes,
        paging: &paging,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "likes/like.html.jinja")]
pub struct LikeTpl<'a> {
    pub like: &'a Like,
    pub index: bool,
    pub atom: bool,
}

#[get("/likes.atom")]
pub async fn index_atom(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let likes = web::block(move || {
        let conn = pool_.get()?;
        actions::list_likes(50, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let newest_like = likes.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_like {
        Some(like) => DateTime::from_utc(like.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = likes
        .iter()
        .map(|like| {
            let fixed_tz = Local.offset_from_utc_datetime(&like.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&like.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_utc(like.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Like/{}", like.id))
                .published(inserted)
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(like_uri(like))
                        .mime_type("text/html".to_owned())
                        .rel("alternate")
                        .build(),
                )
                .title(format!("♥ {}", like.in_reply_to))
                .content(
                    ContentBuilder::default()
                        .content_type("html".to_owned())
                        .value(
                            LikeTpl {
                                like: &like,
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
        .lang("en-US".to_owned())
        .id(likes_atom_uri())
        .title("WWWTech / Likes")
        .link(
            LinkBuilder::default()
                .href(likes_uri())
                .mime_type("text/html".to_owned())
                .rel("alternate")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(likes_atom_uri())
                .mime_type("application/atom+xml".to_owned())
                .rel("self")
                .build(),
        )
        .updated(updated_at)
        .author(
            PersonBuilder::default()
                .name("Christian Kruse")
                .email("christian@kruse.cool".to_owned())
                .uri("https://wwwtech.de/about".to_owned())
                .build(),
        )
        .entries(entries)
        .build()
        .to_string();

    Ok(HttpResponse::Ok()
        .content_type("application/atom+xml; charset=utf-8")
        .body(s))
}
