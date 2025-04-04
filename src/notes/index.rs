use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{Html, IntoResponse};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use super::{PER_PAGE, actions};
use crate::errors::AppError;
use crate::models::Note;
use crate::uri_helpers::*;
use crate::utils::paging::{PageParams, Paging, get_page, get_paging};
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "notes/index.html.j2")]
pub struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    notes: Vec<Vec<Note>>,
    paging: Paging,
    index: bool,
    atom: bool,
}

pub async fn index(
    auth: AuthSession,
    State(state): State<AppState>,
    page: Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let p = get_page(&page.0);

    let logged_in = auth.user.is_some();
    let mut conn = state.pool.acquire().await?;
    let notes = actions::list_notes(PER_PAGE, p * PER_PAGE, !logged_in, &mut conn).await?;
    let count = actions::count_notes(!logged_in, &mut conn).await?;

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

    let html = Index {
        lang: "en",
        title: Some("Notes"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in,
        notes: grouped_notes,
        paging,
        index: true,
        atom: false,
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "notes/note.html.j2")]
pub struct NoteTpl<'a> {
    pub note: &'a Note,
    pub index: bool,
    pub atom: bool,
}

pub async fn index_atom(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let notes = actions::list_notes(50, 0, true, &mut conn).await?;

    let newest_note = notes.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_note {
        Some(note) => DateTime::from_naive_utc_and_offset(note.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = notes
        .iter()
        .map(|note| {
            let fixed_tz = Local.offset_from_utc_datetime(&note.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&note.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_naive_utc_and_offset(note.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Note/{}", note.id))
                .published(Some(inserted))
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(note_uri(note))
                        .mime_type(Some("text/html".to_owned()))
                        .rel("alternate".to_owned())
                        .build(),
                )
                .title(note.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type(Some("html".to_owned()))
                        .value(
                            NoteTpl {
                                note,
                                index: false,
                                atom: true,
                            }
                            .render()
                            .ok(),
                        )
                        .build(),
                )
                .build()
        })
        .collect();

    let s = FeedBuilder::default()
        .lang(Some("en-US".to_owned()))
        .id(notes_atom_uri())
        .title("WWWTech / Notes")
        .link(
            LinkBuilder::default()
                .href(notes_uri())
                .mime_type(Some("text/html".to_owned()))
                .rel("alternate".to_owned())
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(notes_atom_uri())
                .mime_type(Some("application/atom+xml".to_owned()))
                .rel("self".to_owned())
                .build(),
        )
        .updated(updated_at)
        .author(
            PersonBuilder::default()
                .name("Christian Kruse".to_owned())
                .email(Some("christian@kruse.cool".to_owned()))
                .uri(Some("https://wwwtech.de/about".to_owned()))
                .build(),
        )
        .entries(entries)
        .build()
        .to_string();

    Ok(([(header::CONTENT_TYPE, "application/atom+xml; charset=utf-8")], s))
}
