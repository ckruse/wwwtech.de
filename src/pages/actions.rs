use chrono::{NaiveDateTime, Utc};
use sqlx::PgConnection;

use crate::{
    likes::actions as like_actions,
    models::{Article, Like, Note, Picture},
    notes::actions as note_actions,
    pictures::actions as picture_actions,
};

#[derive(Clone)]
pub enum NotePictureLike {
    Note(Note),
    Picture(Picture),
    Like(Like),
    Article(Article),
    None,
}

pub fn inserted_at_for(itm: &NotePictureLike) -> NaiveDateTime {
    match itm {
        NotePictureLike::Note(n) => n.inserted_at,
        NotePictureLike::Picture(p) => p.inserted_at,
        NotePictureLike::Like(l) => l.inserted_at,
        NotePictureLike::Article(a) => a.inserted_at,
        NotePictureLike::None => Utc::now().naive_utc(),
    }
}

pub fn updated_at_for(itm: &NotePictureLike) -> NaiveDateTime {
    match itm {
        NotePictureLike::Note(n) => n.updated_at,
        NotePictureLike::Picture(p) => p.updated_at,
        NotePictureLike::Like(l) => l.updated_at,
        NotePictureLike::Article(a) => a.updated_at,
        NotePictureLike::None => Utc::now().naive_utc(),
    }
}

pub async fn get_last_ten_items(conn: &mut PgConnection) -> Result<Vec<NotePictureLike>, sqlx::Error> {
    let notes = note_actions::list_notes(10, 0, true, &mut *conn).await?;
    let pictures = picture_actions::list_pictures(10, 0, true, &mut *conn).await?;
    let likes = like_actions::list_likes(10, 0, true, &mut *conn).await?;

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
        let dt_a = inserted_at_for(a);
        let dt_b = inserted_at_for(b);

        dt_b.partial_cmp(&dt_a).unwrap()
    });

    items.resize(10, NotePictureLike::None);

    Ok(items)
}
