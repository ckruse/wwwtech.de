use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use axum::extract::State;
use axum::http::header;
use axum::response::{Html, IntoResponse};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use super::actions;
use super::actions::NotePictureLike;
use crate::articles::actions as article_actions;
use crate::deafies::actions as deafie_actions;
use crate::errors::AppError;
use crate::models::{Article, Deafie};
use crate::uri_helpers::*;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "pages/index.html.j2")]
pub struct Index<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    home: bool,
    index: bool,
    atom: bool,
    picture_type: &'a str,

    article: Article,
    deafie: Deafie,
    items: Vec<Vec<NotePictureLike>>,
}

pub async fn index(auth: AuthSession, State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let article = article_actions::get_youngest_article(true, &mut conn).await?;
    let deafie = deafie_actions::get_youngest_deafie(true, &mut conn).await?;
    let items = actions::get_last_ten_items(&mut conn).await?;

    let grouped_items: Vec<Vec<NotePictureLike>> = {
        let mut groups = Vec::new();
        let mut this_group: Vec<NotePictureLike> = Vec::new();

        for item in items {
            if this_group.is_empty()
                || actions::inserted_at_for(&this_group[0]).date() == actions::inserted_at_for(&item).date()
            {
                this_group.push(item);
            } else {
                groups.push(this_group);
                this_group = vec![item];
            }
        }
        groups
    };

    let html = Index {
        lang: "en",
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: auth.user.is_some(),

        home: true,
        index: true,
        atom: false,
        picture_type: "thumbnail",

        article,
        deafie,
        items: grouped_items,
    }
    .render()?;

    Ok(Html(html))
}

pub async fn index_atom(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let article = article_actions::get_youngest_article(true, &mut conn).await?;
    let mut items = actions::get_last_ten_items(&mut conn).await?;

    items.push(NotePictureLike::Article(article));

    items.sort_by(|a, b| {
        let dt_a = actions::inserted_at_for(a);
        let dt_b = actions::inserted_at_for(b);

        dt_b.partial_cmp(&dt_a).unwrap()
    });

    let newest_item = items
        .iter()
        .min_by(|a, b| actions::updated_at_for(a).cmp(&actions::updated_at_for(b)))
        .unwrap();
    let updated_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(actions::updated_at_for(newest_item), Utc);

    let entries: Vec<Entry> = items
        .iter()
        .map(|item| {
            let inserted_at = actions::inserted_at_for(item);
            let fixed_tz = Local.offset_from_utc_datetime(&inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&inserted_at);
            let updated: DateTime<Utc> = DateTime::from_naive_utc_and_offset(actions::updated_at_for(item), Utc);

            let (id, title, uri, content) = match item {
                NotePictureLike::Article(article) => Some((
                    format!("tag:wwwtech.de,2005:Article/{}", article.id),
                    article.title.clone(),
                    article_uri(article),
                    crate::articles::index::ArticleTpl {
                        article,
                        index: false,
                        atom: true,
                    }
                    .render()
                    .ok(),
                )),
                NotePictureLike::Note(note) => Some((
                    format!("tag:wwwtech.de,2005:Note/{}", note.id),
                    note.title.clone(),
                    note_uri(note),
                    crate::notes::index::NoteTpl {
                        note,
                        index: false,
                        atom: true,
                    }
                    .render()
                    .ok(),
                )),
                NotePictureLike::Picture(picture) => Some((
                    format!("tag:wwwtech.de,2005:Picture/{}", picture.id),
                    picture.title.clone(),
                    picture_uri(picture),
                    crate::pictures::index::PictureTpl {
                        picture,
                        picture_type: "thumbnail",
                        index: false,
                        atom: true,
                        home: false,
                    }
                    .render()
                    .ok(),
                )),
                NotePictureLike::Like(like) => Some((
                    format!("tag:wwwtech.de,2005:Like/{}", like.id),
                    format!("♥ {}", like.in_reply_to),
                    like_uri(like),
                    crate::likes::index::LikeTpl {
                        like,
                        index: false,
                        atom: true,
                    }
                    .render()
                    .ok(),
                )),
                NotePictureLike::None => None,
            }
            .unwrap();

            EntryBuilder::default()
                .id(id)
                .published(Some(inserted))
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(uri)
                        .mime_type(Some("text/html".to_owned()))
                        .rel("alternate".to_owned())
                        .build(),
                )
                .title(title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type(Some("html".to_owned()))
                        .value(content)
                        .build(),
                )
                .build()
        })
        .collect();

    let s = FeedBuilder::default()
        .lang(Some("en-US".to_owned()))
        .id(whatsnew_atom_uri())
        .title("WWWTech / What’s new? (Combined feed)")
        .link(
            LinkBuilder::default()
                .href(root_uri())
                .mime_type(Some("text/html".to_owned()))
                .rel("alternate".to_owned())
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(whatsnew_atom_uri())
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
