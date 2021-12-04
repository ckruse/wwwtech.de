use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::vec::Vec;
use validator::{Validate, ValidationError};

use crate::models::{NewNote, Note};
use crate::DbError;

pub fn list_notes(limit: i64, offset: i64, only_visible: bool, conn: &PgConnection) -> Result<Vec<Note>, DbError> {
    use crate::schema::notes::dsl::*;

    let notes_list = notes
        .filter(show_in_index.eq(only_visible))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .load::<Note>(conn)?;

    Ok(notes_list)
}

pub fn count_notes(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::notes::dsl::*;
    use diesel::dsl::count;

    let cnt = notes
        .filter(show_in_index.eq(only_visible))
        .select(count(id))
        .first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_note(note_id: i32, only_visible: bool, conn: &PgConnection) -> Result<Note, DbError> {
    use crate::schema::notes::dsl::*;

    let note = notes
        .filter(show_in_index.eq(only_visible))
        .filter(id.eq(note_id))
        .first::<Note>(conn)?;

    Ok(note)
}

pub fn create_note(data: &NewNote, conn: &PgConnection) -> Result<Note, DbError> {
    use crate::schema::notes;
    use diesel::select;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);

    if data.in_reply_to == Some("".to_string()) {
        data.in_reply_to = None;
    }

    if data.content.is_none() || data.content == Some("".to_string()) {
        data.content = Some(data.title.clone());
    }

    match data.validate() {
        Ok(_) => {
            let note = diesel::insert_into(notes::table)
                .values(data)
                .get_result::<Note>(conn)?;

            Ok(note)
        }
        Err(errors) => Err(Box::new(errors)),
    }
}
