use actix_web::{error, web, Error};
use chrono::Utc;

use crate::models::{Article, Like, Note, Picture};
use crate::DbPool;

use crate::likes::actions as like_actions;
use crate::notes::actions as note_actions;
use crate::pictures::actions as picture_actions;

#[derive(Clone)]
pub enum NotePictureLike {
    Note(Note),
    Picture(Picture),
    Like(Like),
    Article(Article),
    None,
}

pub fn inserted_at_for(itm: &NotePictureLike) -> chrono::NaiveDateTime {
    match itm {
        NotePictureLike::Note(n) => n.inserted_at,
        NotePictureLike::Picture(p) => p.inserted_at,
        NotePictureLike::Like(l) => l.inserted_at,
        NotePictureLike::Article(a) => a.inserted_at,
        NotePictureLike::None => Utc::now().naive_utc(),
    }
}

pub fn updated_at_for(itm: &NotePictureLike) -> chrono::NaiveDateTime {
    match itm {
        NotePictureLike::Note(n) => n.updated_at,
        NotePictureLike::Picture(p) => p.updated_at,
        NotePictureLike::Like(l) => l.updated_at,
        NotePictureLike::Article(a) => a.updated_at,
        NotePictureLike::None => Utc::now().naive_utc(),
    }
}

pub async fn get_last_ten_items(pool: &web::Data<DbPool>) -> Result<Vec<NotePictureLike>, Error> {
    let pool_ = pool.clone();
    let notes = web::block(move || {
        let conn = pool_.get()?;
        note_actions::list_notes(10, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let pool_ = pool.clone();
    let pictures = web::block(move || {
        let conn = pool_.get()?;
        picture_actions::list_pictures(10, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let pool_ = pool.clone();
    let likes = web::block(move || {
        let conn = pool_.get()?;
        like_actions::list_likes(10, 0, true, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut items: Vec<NotePictureLike> = Vec::with_capacity(30);
    for note in notes.into_iter() {
        let v = NotePictureLike::Note(note);
        items.push(v);
    }

    for picture in pictures.into_iter() {
        let v = NotePictureLike::Picture(picture);
        items.push(v);
    }

    for like in likes.into_iter() {
        let v = NotePictureLike::Like(like);
        items.push(v);
    }

    items.sort_by(|a, b| {
        let dt_a = inserted_at_for(&a);
        let dt_b = inserted_at_for(&b);

        dt_b.partial_cmp(&dt_a).unwrap()
    });

    items.resize(10, NotePictureLike::None);

    Ok(items)
}
