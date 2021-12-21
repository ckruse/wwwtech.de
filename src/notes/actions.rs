use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::vec::Vec;
use validator::Validate;

use crate::models::{NewNote, Note};
use crate::DbError;

pub fn list_notes(limit: i64, offset: i64, only_visible: bool, conn: &PgConnection) -> Result<Vec<Note>, DbError> {
    use crate::schema::notes::dsl::*;

    let mut notes_list_query = notes
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        notes_list_query = notes_list_query.filter(show_in_index.eq(only_visible));
    }

    let notes_list = notes_list_query.load::<Note>(conn)?;

    Ok(notes_list)
}

pub fn count_notes(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::notes::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = notes.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(show_in_index.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_note(note_id: i32, conn: &PgConnection) -> Result<Note, DbError> {
    use crate::schema::notes::dsl::*;

    let note = notes.filter(id.eq(note_id)).first::<Note>(conn)?;

    Ok(note)
}

pub fn create_note(data: &NewNote, conn: &PgConnection) -> Result<Note, DbError> {
    use crate::schema::notes;
    use diesel::select;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let note = diesel::insert_into(notes::table)
            .values(data)
            .get_result::<Note>(conn)?;

        Ok(note)
    }
}

pub fn update_note(note_id: i32, data: &NewNote, conn: &PgConnection) -> Result<Note, DbError> {
    use crate::schema::notes::dsl::*;
    use diesel::select;

    let mut data = data.clone();

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
        let note = diesel::update(notes.find(note_id))
            .set((
                title.eq(data.title),
                lang.eq(data.lang),
                in_reply_to.eq(data.in_reply_to),
                posse.eq(data.posse),
                show_in_index.eq(data.show_in_index),
                content.eq(data.content.unwrap()),
                updated_at.eq(now),
            ))
            .get_result::<Note>(conn)?;

        Ok(note)
    }
}

pub fn delete_note(note_id: i32, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::mentions;
    use crate::schema::notes::dsl::*;

    let num_deleted = conn.transaction(move || {
        diesel::delete(mentions::table.filter(mentions::note_id.eq(note_id))).execute(conn)?;
        diesel::delete(notes.filter(id.eq(note_id))).execute(conn)
    })?;

    Ok(num_deleted)
}
