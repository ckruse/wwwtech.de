use diesel::prelude::*;
use std::vec::Vec;

use crate::models::Picture;
use crate::DbError;

pub fn list_pictures(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &PgConnection,
) -> Result<Vec<Picture>, DbError> {
    use crate::schema::pictures::dsl::*;

    let pictures_list = pictures
        .filter(show_in_index.eq(only_visible))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .load::<Picture>(conn)?;

    Ok(pictures_list)
}

pub fn count_pictures(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::pictures::dsl::*;
    use diesel::dsl::count;

    let cnt = pictures
        .filter(show_in_index.eq(only_visible))
        .select(count(id))
        .first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_picture(oicture_id: i32, only_visible: bool, conn: &PgConnection) -> Result<Picture, DbError> {
    use crate::schema::pictures::dsl::*;

    let picture = pictures
        .filter(show_in_index.eq(only_visible))
        .filter(id.eq(oicture_id))
        .first::<Picture>(conn)?;

    Ok(picture)
}
