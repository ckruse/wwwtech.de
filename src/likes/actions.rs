use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::vec::Vec;
use validator::Validate;

use crate::models::{Like, NewLike};
use crate::DbError;

pub fn list_likes(limit: i64, offset: i64, only_visible: bool, conn: &mut PgConnection) -> Result<Vec<Like>, DbError> {
    use crate::schema::likes::dsl::*;

    let mut likes_list_query = likes
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        likes_list_query = likes_list_query.filter(show_in_index.eq(only_visible));
    }

    let likes_list = likes_list_query.load::<Like>(conn)?;

    Ok(likes_list)
}

pub fn count_likes(only_visible: bool, conn: &mut PgConnection) -> Result<i64, DbError> {
    use crate::schema::likes::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = likes.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(show_in_index.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_like(like_id: i32, conn: &mut PgConnection) -> Result<Like, DbError> {
    use crate::schema::likes::dsl::*;

    let like = likes.filter(id.eq(like_id)).first::<Like>(conn)?;

    Ok(like)
}

pub fn create_like(data: &NewLike, conn: &mut PgConnection) -> Result<Like, DbError> {
    use crate::schema::likes;
    use diesel::select;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let like = diesel::insert_into(likes::table)
            .values(data)
            .get_result::<Like>(conn)?;

        Ok(like)
    }
}

pub fn update_like(like_id: i32, data: &NewLike, conn: &mut PgConnection) -> Result<Like, DbError> {
    use crate::schema::likes::dsl::*;
    use diesel::select;

    let data = data.clone();

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
        let like = diesel::update(likes.find(like_id))
            .set((
                in_reply_to.eq(data.in_reply_to),
                posse.eq(data.posse),
                show_in_index.eq(data.show_in_index),
                updated_at.eq(now),
            ))
            .get_result::<Like>(conn)?;

        Ok(like)
    }
}

pub fn delete_like(like_id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::likes::dsl::*;

    let num_deleted = diesel::delete(likes.filter(id.eq(like_id))).execute(conn)?;

    Ok(num_deleted)
}
