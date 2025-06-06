use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{Html, IntoResponse};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use super::{PER_PAGE, actions};
use crate::errors::AppError;
use crate::models::Picture;
use crate::uri_helpers::*;
use crate::utils::paging::{PageParams, Paging, get_page, get_paging};
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "pictures/index.html.j2")]
pub struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    pictures: Vec<Picture>,
    paging: Paging,
    index: bool,
    atom: bool,
    home: bool,
    picture_type: &'a str,
}

pub async fn index(
    auth: AuthSession,
    State(state): State<AppState>,
    page: Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let p = get_page(&page);
    let only_visible = auth.user.is_none();
    let mut conn = state.pool.acquire().await?;
    let pictures = actions::list_pictures(PER_PAGE, p * PER_PAGE, only_visible, &mut conn).await?;
    let count = actions::count_pictures(only_visible, &mut conn).await?;

    let paging = get_paging(count, p, PER_PAGE);

    let html = Index {
        lang: "en",
        title: Some("Pictures"),
        page_type: None,
        page_image: None,
        body_id: Some("pictures-list"),
        logged_in: !only_visible,
        pictures,
        paging,
        index: true,
        atom: false,
        home: false,
        picture_type: "thumbnail",
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "pictures/picture.html.j2")]
pub struct PictureTpl<'a> {
    pub picture: &'a Picture,
    pub index: bool,
    pub atom: bool,
    pub home: bool,
    pub picture_type: &'a str,
}

pub async fn index_atom(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let pictures = actions::list_pictures(50, 0, true, &mut conn).await?;

    let newest_picture = pictures.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_picture {
        Some(picture) => DateTime::from_naive_utc_and_offset(picture.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = pictures
        .iter()
        .map(|picture| {
            let fixed_tz = Local.offset_from_utc_datetime(&picture.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&picture.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_naive_utc_and_offset(picture.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Picture/{}", picture.id))
                .published(Some(inserted))
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(picture_uri(picture))
                        .mime_type(Some("text/html".to_owned()))
                        .rel("alternate".to_owned())
                        .build(),
                )
                .title(picture.title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type(Some("html".to_owned()))
                        .value(
                            PictureTpl {
                                picture,
                                picture_type: "thumbnail",
                                index: false,
                                atom: true,
                                home: false,
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
        .id(pictures_atom_uri())
        .title("WWWTech / Pictures")
        .link(
            LinkBuilder::default()
                .href(pictures_uri())
                .mime_type(Some("text/html".to_owned()))
                .rel("alternate".to_owned())
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(pictures_atom_uri())
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
