use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use super::{actions, PER_PAGE};
use crate::{
    errors::AppError,
    models::Picture,
    uri_helpers::*,
    utils as filters,
    utils::paging::{get_page, get_paging, PageParams, Paging},
    AppState, AuthContext,
};

#[derive(Template)]
#[template(path = "pictures/index.html.jinja")]
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
    auth: AuthContext,
    State(state): State<AppState>,
    page: Query<PageParams>,
) -> Result<Index<'static>, AppError> {
    let p = get_page(&page);
    let only_visible = auth.current_user.is_none();
    let mut conn = state.pool.acquire().await?;
    let pictures = actions::list_pictures(PER_PAGE, p * PER_PAGE, only_visible, &mut conn).await?;
    let count = actions::count_pictures(only_visible, &mut conn).await?;

    let paging = get_paging(count, p, PER_PAGE);

    Ok(Index {
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
    })
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

pub async fn index_atom(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let pictures = actions::list_pictures(50, 0, true, &mut conn).await?;

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
