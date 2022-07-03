use anyhow::Result;
use chrono::{Datelike, NaiveDateTime};
use diesel::prelude::*;
use std::vec::Vec;
use validator::Validate;

use crate::models::{Deafie, NewDeafie};
use crate::uri_helpers::root_uri;
use crate::utils::MONTHS;
use crate::DbError;

pub fn list_deafies(limit: i64, offset: i64, only_visible: bool, conn: &PgConnection) -> Result<Vec<Deafie>, DbError> {
    use crate::schema::deafies::dsl::*;

    let mut deafie_list_query = deafies
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        deafie_list_query = deafie_list_query.filter(published.eq(only_visible));
    }

    let deafie_list = deafie_list_query.load::<Deafie>(conn)?;

    Ok(deafie_list)
}

pub fn count_deafies(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::deafies::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = deafies.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(published.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_deafie(deafie_id: i32, only_visible: bool, conn: &PgConnection) -> Result<Deafie> {
    use crate::schema::deafies::dsl::*;

    let mut deafie_query = deafies.filter(id.eq(deafie_id)).into_boxed();

    if only_visible {
        deafie_query = deafie_query.filter(published.eq(only_visible));
    }

    let deafie = deafie_query.first::<Deafie>(conn)?;

    Ok(deafie)
}

pub fn get_deafie_by_slug(deafie_slug: &str, only_visible: bool, conn: &PgConnection) -> Result<Deafie> {
    use crate::schema::deafies::dsl::*;

    let mut deafie_query = deafies.filter(slug.eq(deafie_slug)).into_boxed();

    if only_visible {
        deafie_query = deafie_query.filter(published.eq(only_visible));
    }

    let deafie = deafie_query.first::<Deafie>(conn)?;

    Ok(deafie)
}

pub fn create_deafie(data: &NewDeafie, conn: &PgConnection) -> Result<Deafie> {
    use crate::schema::deafies;
    use diesel::select;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);

    let mon_idx = usize::try_from(now.month0()).unwrap();
    let mut guid = String::new();
    guid.push_str(&now.year().to_string());
    guid.push_str("/");
    guid.push_str(MONTHS[mon_idx]);
    guid.push_str("/");
    guid.push_str(&data.slug);
    data.guid = Some(root_uri() + &guid.clone());
    data.slug = guid;

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    if let Err(errors) = data.validate() {
        Err(anyhow::Error::from(errors))
    } else {
        let deafie = diesel::insert_into(deafies::table)
            .values(data)
            .get_result::<Deafie>(conn)?;

        Ok(deafie)
    }
}

pub fn update_deafie(deafie_id: i32, data: &NewDeafie, conn: &PgConnection) -> Result<Deafie> {
    use crate::schema::deafies::dsl::*;
    use diesel::select;

    let mut data = data.clone();

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    if let Err(errors) = data.validate() {
        Err(anyhow::Error::from(errors))
    } else {
        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
        let deafie = diesel::update(deafies.find(deafie_id))
            .set((
                title.eq(data.title),
                slug.eq(data.slug),
                excerpt.eq(data.excerpt),
                body.eq(data.body),
                published.eq(data.published),
                updated_at.eq(now),
            ))
            .get_result::<Deafie>(conn)?;

        Ok(deafie)
    }
}

pub fn delete_deafie(deafie_id: i32, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::deafies::dsl::*;
    use crate::schema::mentions;

    let num_deleted = conn.transaction(move || {
        diesel::delete(mentions::table.filter(mentions::deafie_id.eq(deafie_id))).execute(conn)?;
        diesel::delete(deafies.filter(id.eq(deafie_id))).execute(conn)
    })?;

    Ok(num_deleted)
}
