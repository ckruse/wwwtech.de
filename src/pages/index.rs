use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use atom_syndication::{ContentBuilder, Entry, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

use crate::models::Article;
use crate::models::Deafie;

use crate::DbPool;

use crate::articles::actions as article_actions;
use crate::deafies::actions as deafie_actions;

use super::actions;
use super::actions::NotePictureLike;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "pages/index.html.jinja")]
struct Index<'a> {
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

    article: &'a Article,
    deafie: &'a Deafie,
    items: &'a Vec<Vec<NotePictureLike>>,
}

#[get("/")]
pub async fn index(id: Option<Identity>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let article = web::block(move || {
        let mut conn = pool_.get()?;
        article_actions::get_youngest_article(true, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let pool_ = pool.clone();
    let deafie = web::block(move || {
        let mut conn = pool_.get()?;
        deafie_actions::get_youngest_deafie(true, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let items = actions::get_last_ten_items(&pool).await?;

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

    let s = Index {
        lang: "en",
        title: None,
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.is_some(),

        home: true,
        index: true,
        atom: false,
        picture_type: "thumbnail",

        article: &article,
        deafie: &deafie,
        items: &grouped_items,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[get("/whatsnew.atom")]
pub async fn index_atom(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let article = web::block(move || {
        let mut conn = pool_.get()?;
        article_actions::get_youngest_article(true, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut items = actions::get_last_ten_items(&pool).await?;
    items.push(NotePictureLike::Article(article));

    items.sort_by(|a, b| {
        let dt_a = actions::inserted_at_for(&a);
        let dt_b = actions::inserted_at_for(&b);

        dt_b.partial_cmp(&dt_a).unwrap()
    });

    let newest_item = items
        .iter()
        .min_by(|a, b| actions::updated_at_for(a).cmp(&actions::updated_at_for(b)))
        .unwrap();
    let updated_at: DateTime<Utc> = DateTime::from_utc(actions::updated_at_for(newest_item), Utc);

    let entries: Vec<Entry> = items
        .iter()
        .map(|item| {
            let inserted_at = actions::inserted_at_for(item);
            let fixed_tz = Local.offset_from_utc_datetime(&inserted_at);
            let inserted: DateTime<FixedOffset> = fixed_tz.from_utc_datetime(&inserted_at);
            let updated: DateTime<Utc> = DateTime::from_utc(actions::updated_at_for(item), Utc);

            let (id, title, uri, content) = match item {
                NotePictureLike::Article(article) => Some((
                    format!("tag:wwwtech.de,2005:Article/{}", article.id),
                    article.title.clone(),
                    article_uri(article),
                    crate::articles::index::ArticleTpl {
                        article: &article,
                        index: false,
                        atom: true,
                    }
                    .render()
                    .unwrap(),
                )),
                NotePictureLike::Note(note) => Some((
                    format!("tag:wwwtech.de,2005:Note/{}", note.id),
                    note.title.clone(),
                    note_uri(note),
                    crate::notes::index::NoteTpl {
                        note: &note,
                        index: false,
                        atom: true,
                    }
                    .render()
                    .unwrap(),
                )),
                NotePictureLike::Picture(picture) => Some((
                    format!("tag:wwwtech.de,2005:Picture/{}", picture.id),
                    picture.title.clone(),
                    picture_uri(picture),
                    crate::pictures::index::PictureTpl {
                        picture: &picture,
                        picture_type: "thumbnail",
                        index: false,
                        atom: true,
                        home: false,
                    }
                    .render()
                    .unwrap(),
                )),
                NotePictureLike::Like(like) => Some((
                    format!("tag:wwwtech.de,2005:Like/{}", like.id),
                    format!("♥ {}", like.in_reply_to),
                    like_uri(like),
                    crate::likes::index::LikeTpl {
                        like: &like,
                        index: false,
                        atom: true,
                    }
                    .render()
                    .unwrap(),
                )),
                NotePictureLike::None => None,
            }
            .unwrap();

            EntryBuilder::default()
                .id(id)
                .published(inserted)
                .updated(updated)
                .link(
                    LinkBuilder::default()
                        .href(uri)
                        .mime_type("text/html".to_owned())
                        .rel("alternate")
                        .build(),
                )
                .title(title.as_str())
                .content(
                    ContentBuilder::default()
                        .content_type("html".to_owned())
                        .value(content)
                        .build(),
                )
                .build()
        })
        .collect();

    let s = FeedBuilder::default()
        .lang("en-US".to_owned())
        .id(whatsnew_atom_uri())
        .title("WWWTech / What’s new? (Combined feed)")
        .link(
            LinkBuilder::default()
                .href(root_uri())
                .mime_type("text/html".to_owned())
                .rel("alternate")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .href(whatsnew_atom_uri())
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
