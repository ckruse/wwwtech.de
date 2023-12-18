use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use axum::{
    extract::{Query, State},
    http::header,
    response::IntoResponse,
};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use super::{actions, PER_PAGE};
use crate::{
    errors::AppError,
    models::Deafie,
    uri_helpers::*,
    utils as filters,
    utils::paging::{get_page, get_paging, PageParams, Paging},
    AppState, AuthSession,
};

#[derive(Template)]
#[template(path = "deafies/index.html.jinja")]
pub struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    deafies: Vec<Deafie>,
    paging: Paging,
    index: bool,
    atom: bool,
    // home: bool,
}

pub async fn index(
    auth: AuthSession,
    State(state): State<AppState>,
    page: Query<PageParams>,
) -> Result<Index<'static>, AppError> {
    let p = get_page(&page);

    let logged_in = auth.user.is_some();
    let mut conn = state.pool.acquire().await?;
    let deafies = actions::list_deafies(PER_PAGE, p * PER_PAGE, !logged_in, &mut conn).await?;
    let count = actions::count_deafies(!logged_in, &mut conn).await?;
    let paging = get_paging(count, p, PER_PAGE);

    Ok(Index {
        lang: "de",
        title: Some("The Life of Alfons: das Leben mit einem gehörlosen Hund"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in,
        deafies,
        paging,
        index: true,
        atom: false,
        // home: false,
    })
}

#[derive(Template)]
#[template(path = "deafies/deafie.html.jinja")]
pub struct DeafieTpl<'a> {
    pub deafie: &'a Deafie,
    pub index: bool,
    pub atom: bool,
}

pub async fn index_atom(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let logged_in = false;
    let mut conn = state.pool.acquire().await?;
    let deafies = actions::list_deafies(50, 0, !logged_in, &mut conn).await?;

    let newest_deafie = deafies.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_deafie {
        Some(deafie) => DateTime::from_naive_utc_and_offset(deafie.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = deafies
        .iter()
        .map(|deafie| {
            let fixed_tz = Local.offset_from_utc_datetime(&deafie.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&deafie.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_naive_utc_and_offset(deafie.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2022:Deafie/{}", deafie.id))
                .published(Some(inserted))
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(deafie_uri(deafie))
                        .mime_type(Some("text/html".to_owned()))
                        .rel("alternate".to_owned())
                        .build(),
                )
                .title(deafie.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type(Some("html".to_owned()))
                        .value(
                            DeafieTpl {
                                deafie,
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
        .lang(Some("de-DE".to_owned()))
        .id(deafies_atom_uri())
        .title("WWWTech / einen gehörlosen Hund ausbilden")
        .link(
            LinkBuilder::default()
                .href(deafies_uri())
                .mime_type(Some("text/html".to_owned()))
                .rel("alternate".to_owned())
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(deafies_atom_uri())
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
