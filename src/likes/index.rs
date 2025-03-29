use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{Html, IntoResponse};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use super::{PER_PAGE, actions};
use crate::errors::AppError;
use crate::models::Like;
use crate::uri_helpers::*;
use crate::utils::paging::{PageParams, Paging, get_page, get_paging};
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "likes/index.html.j2")]
pub struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    likes: Vec<Like>,
    paging: Paging,
    index: bool,
    atom: bool,
}

pub async fn index(
    auth: AuthSession,
    State(state): State<AppState>,
    page: Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let p = get_page(&page);

    let logged_in = auth.user.is_some();
    let mut conn = state.pool.acquire().await?;

    let likes = actions::list_likes(PER_PAGE, p * PER_PAGE, !logged_in, &mut conn).await?;
    let count = actions::count_likes(true, &mut conn).await?;

    let paging = get_paging(count, p, PER_PAGE);

    let html = Index {
        lang: "en",
        title: Some("Likes"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in,
        likes,
        paging,
        index: true,
        atom: false,
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "likes/like.html.j2")]
pub struct LikeTpl<'a> {
    pub like: &'a Like,
    pub index: bool,
    pub atom: bool,
}

pub async fn index_atom(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let likes = actions::list_likes(50, 0, true, &mut conn).await?;

    let newest_like = likes.iter().min_by(|a, b| a.updated_at.cmp(&b.updated_at));
    let updated_at: DateTime<Utc> = match newest_like {
        Some(like) => DateTime::from_naive_utc_and_offset(like.updated_at, Utc),
        None => Utc::now(),
    };

    let entries: Vec<Entry> = likes
        .iter()
        .map(|like| {
            let fixed_tz = Local.offset_from_utc_datetime(&like.inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&like.inserted_at);
            let updated: DateTime<Utc> = DateTime::from_naive_utc_and_offset(like.updated_at, Utc);
            EntryBuilder::default()
                .id(format!("tag:wwwtech.de,2005:Like/{}", like.id))
                .published(Some(inserted))
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(like_uri(like))
                        .mime_type(Some("text/html".to_owned()))
                        .rel("alternate".to_owned())
                        .build(),
                )
                .title(format!("♥ {}", like.in_reply_to))
                .content(
                    ContentBuilder::default()
                        .content_type(Some("html".to_owned()))
                        .value(
                            LikeTpl {
                                like,
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
        .id(likes_atom_uri())
        .title("WWWTech / Likes")
        .link(
            LinkBuilder::default()
                .href(likes_uri())
                .mime_type(Some("text/html".to_owned()))
                .rel("alternate".to_owned())
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(likes_atom_uri())
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
