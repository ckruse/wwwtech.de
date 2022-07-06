use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

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

#[derive(Template)]
#[template(path = "deafies/deafie.html.jinja")]
pub struct DeafieTpl<'a> {
    pub deafie: &'a Deafie,
    pub index: bool,
    pub atom: bool,
}

#[get("/deaf-dog-training.atom")]
pub async fn index_atom(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let deafies = web::block(move || {
        let conn = pool_.get()?;
        actions::list_deafies(50, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let newest_deafie = deafies.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_deafie {
        Some(deafie) => DateTime::from_utc(deafie.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = deafies
        .iter()
        .map(|deafie| {
            let fixed_tz = Local.offset_from_utc_datetime(&deafie.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&deafie.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_utc(deafie.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2022:Deafie/{}", deafie.id))
                .published(inserted)
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(deafie_uri(deafie))
                        .mime_type("text/html".to_owned())
                        .rel("alternate")
                        .build(),
                )
                .title(deafie.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type("html".to_owned())
                        .value(
                            DeafieTpl {
                                deafie: &deafie,
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
        .lang("de-DE".to_owned())
        .id(deafies_atom_uri())
        .title("WWWTech / einen gehörlosen Hund ausbilden")
        .link(
            LinkBuilder::default()
                .href(deafies_uri())
                .mime_type("text/html".to_owned())
                .rel("alternate")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(deafies_atom_uri())
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
