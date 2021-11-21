use diesel::prelude::*;
use std::vec::Vec;

use crate::models::Like;
use crate::DbError;

pub fn list_likes(limit: i64, offset: i64, only_visible: bool, conn: &PgConnection) -> Result<Vec<Like>, DbError> {
    use crate::schema::likes::dsl::*;

    let likes_list = likes
        .filter(show_in_index.eq(only_visible))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .load::<Like>(conn)?;

    Ok(likes_list)
}

pub fn count_likes(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::likes::dsl::*;
    use diesel::dsl::count;

    let cnt = likes
        .filter(show_in_index.eq(only_visible))
        .select(count(id))
        .first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_like(like_id: i32, only_visible: bool, conn: &PgConnection) -> Result<Like, DbError> {
    use crate::schema::likes::dsl::*;

    let like = likes
        .filter(show_in_index.eq(only_visible))
        .filter(id.eq(like_id))
        .first::<Like>(conn)?;

    Ok(like)
}
