use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use crate::models::Note;
use crate::utils::paging::{get_page, get_paging, PageParams, Paging};
use crate::DbPool;

use super::{actions, PER_PAGE};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "notes/index.html.jinja")]
struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    notes: &'a Vec<Vec<Note>>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
}

#[get("")]
pub async fn index(id: Identity, pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let logged_in = id.identity().is_some();
    let pool_ = pool.clone();
    let notes = web::block(move || {
        let conn = pool_.get()?;
        actions::list_notes(PER_PAGE, p * PER_PAGE, !logged_in, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_notes(true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let grouped_notes: Vec<Vec<Note>> = {
        let mut groups = Vec::new();
        let mut this_group: Vec<Note> = Vec::new();

        for note in notes {
            if this_group.is_empty() || this_group[0].inserted_at.date() == note.inserted_at.date() {
                this_group.push(note);
            } else {
                groups.push(this_group);
                this_group = vec![note];
            }
        }
        groups
    };

    let paging = get_paging(count, p, PER_PAGE);

    let s = Index {
        lang: "en",
        title: Some("Notes"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in,
        notes: &grouped_notes,
        paging: &paging,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[derive(Template)]
#[template(path = "notes/note.html.jinja")]
pub struct NoteTpl<'a> {
    pub note: &'a Note,
    pub index: bool,
    pub atom: bool,
}

#[get("/notes.atom")]
pub async fn index_atom(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let notes = web::block(move || {
        let conn = pool_.get()?;
        actions::list_notes(50, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let newest_note = notes.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at)).unwrap();
    let updated_at: DateTime<Utc> = DateTime::from_utc(newest_note.updated_at, Utc);

    let entries: Vec<Entry> = notes
        .iter()
        .map(|note| {
            let fixed_tz = Local.offset_from_utc_datetime(&note.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&note.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_utc(note.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Note/{}", note.id))
                .published(inserted)
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(note_uri(note))
                        .mime_type("text/html".to_owned())
                        .rel("alternate")
                        .build(),
                )
                .title(note.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type("html".to_owned())
                        .value(
                            NoteTpl {
                                note: &note,
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
        .id(notes_atom_uri())
        .title("WWWTech / Notes")
        .link(
            LinkBuilder::default()
                .href(notes_uri())
                .mime_type("text/html".to_owned())
                .rel("alternate")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(notes_atom_uri())
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
