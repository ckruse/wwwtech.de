use diesel::prelude::*;
use std::vec::Vec;

use crate::models::Note;
use crate::DbError;

pub fn list_notes(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &PgConnection,
) -> Result<Vec<Note>, DbError> {
    use crate::schema::notes::dsl::*;

    let notes_list = notes
        .filter(show_in_index.eq(only_visible))
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
