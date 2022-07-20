use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use crate::models::Picture;
use crate::utils::paging::{get_page, get_paging, PageParams, Paging};
use crate::DbPool;

use super::{actions, PER_PAGE};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "pictures/index.html.jinja")]
struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    pictures: &'a Vec<Picture>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
    home: bool,
    picture_type: &'a str,
}

#[get("")]
pub async fn index(
    id: Option<Identity>,
    pool: web::Data<DbPool>,
    page: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let pool_ = pool.clone();
    let only_visible = id.is_none();
    let pictures = web::block(move || {
        let conn = pool_.get()?;
        actions::list_pictures(PER_PAGE, p * PER_PAGE, only_visible, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_pictures(only_visible, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let s = Index {
        lang: "en",
        title: Some("Pictures"),
        page_type: None,
        page_image: None,
        body_id: Some("pictures-list"),
        logged_in: !only_visible,
        pictures: &pictures,
        paging: &paging,
        index: true,
        atom: false,
        home: false,
        picture_type: "thumbnail",
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "pictures/picture.html.jinja")]
pub struct PictureTpl<'a> {
    pub picture: &'a Picture,
    pub index: bool,
    pub atom: bool,
    pub home: bool,
    pub picture_type: &'a str,
}

#[get("/pictures.atom")]
pub async fn index_atom(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let pictures = web::block(move || {
        let conn = pool_.get()?;
        actions::list_pictures(50, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let newest_picture = pictures.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_picture {
        Some(picture) => DateTime::from_utc(picture.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = pictures
        .iter()
        .map(|picture| {
            let fixed_tz = Local.offset_from_utc_datetime(&picture.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&picture.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_utc(picture.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Picture/{}", picture.id))
                .published(inserted)
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(picture_uri(picture))
                        .mime_type("text/html".to_owned())
                        .rel("alternate")
                        .build(),
                )
                .title(picture.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type("html".to_owned())
                        .value(
                            PictureTpl {
                                picture: &picture,
                                picture_type: "thumbnail",
                                index: false,
                                atom: true,
                                home: false,
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
        .id(pictures_atom_uri())
        .title("WWWTech / Pictures")
        .link(
            LinkBuilder::default()
                .href(pictures_uri())
                .mime_type("text/html".to_owned())
                .rel("alternate")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(pictures_atom_uri())
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
