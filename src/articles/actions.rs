use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use std::vec::Vec;
use validator::Validate;

use crate::models::{Article, NewArticle};
use crate::uri_helpers::root_uri;
use crate::utils::MONTHS;

pub fn list_articles(limit: i64, offset: i64, only_visible: bool, conn: &mut PgConnection) -> Result<Vec<Article>> {
    use crate::schema::articles::dsl::*;

    let mut articles_list_query = articles
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        articles_list_query = articles_list_query.filter(published.eq(only_visible));
    }

    let articles_list = articles_list_query.load::<Article>(conn)?;

    Ok(articles_list)
}

pub fn count_articles(only_visible: bool, conn: &mut PgConnection) -> Result<i64> {
    use crate::schema::articles::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = articles.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(published.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_youngest_article(only_visible: bool, conn: &mut PgConnection) -> Result<Article> {
    use crate::schema::articles::dsl::*;
    let mut article_query = articles
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(1)
        .into_boxed();

    if only_visible {
        article_query = article_query.filter(published.eq(only_visible));
    }

    let article = article_query.first::<Article>(conn)?;

    Ok(article)
}

pub fn get_article(article_id: i32, only_visible: bool, conn: &mut PgConnection) -> Result<Article> {
    use crate::schema::articles::dsl::*;

    let mut article_query = articles.filter(id.eq(article_id)).into_boxed();

    if only_visible {
        article_query = article_query.filter(published.eq(only_visible));
    }

    let article = article_query.first::<Article>(conn)?;

    Ok(article)
}

pub fn get_article_by_slug(article_slug: &str, only_visible: bool, conn: &mut PgConnection) -> Result<Article> {
    use crate::schema::articles::dsl::*;

    let mut article_query = articles.filter(slug.eq(article_slug)).into_boxed();

    if only_visible {
        article_query = article_query.filter(published.eq(only_visible));
    }

    let article = article_query.first::<Article>(conn)?;

    Ok(article)
}

pub fn get_article_by_slug_part(article_slug: &str, only_visible: bool, conn: &mut PgConnection) -> Result<Article> {
    use crate::schema::articles::dsl::*;
    let search_str = format!("%/{}", article_slug);
    let mut article_query = articles.filter(slug.like(search_str)).into_boxed();

    if only_visible {
        article_query = article_query.filter(published.eq(only_visible));
    }

    let article = article_query.first::<Article>(conn)?;

    Ok(article)
}

pub fn create_article(data: &NewArticle, conn: &mut PgConnection) -> Result<Article> {
    use crate::schema::articles;
    use diesel::select;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);
    data.article_format = Some("markdown".to_owned());

    let mon_idx = usize::try_from(now.month0()).unwrap();
    let mut guid = String::new();
    guid.push_str(&now.year().to_string());
    guid.push_str("/");
    guid.push_str(MONTHS[mon_idx]);
    guid.push_str("/");
    guid.push_str(&data.slug);
    data.guid = Some(root_uri() + &guid.clone());
    data.slug = guid;

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    if let Err(errors) = data.validate() {
        Err(anyhow::Error::from(errors))
    } else {
        let article = diesel::insert_into(articles::table)
            .values(data)
            .get_result::<Article>(conn)?;

        Ok(article)
    }
}

pub fn update_article(article_id: i32, data: &NewArticle, conn: &mut PgConnection) -> Result<Article> {
    use crate::schema::articles::dsl::*;
    use diesel::select;

    let mut data = data.clone();

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    if let Err(errors) = data.validate() {
        Err(anyhow::Error::from(errors))
    } else {
        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
        let note = diesel::update(articles.find(article_id))
            .set((
                in_reply_to.eq(data.in_reply_to),
                title.eq(data.title),
                slug.eq(data.slug),
                excerpt.eq(data.excerpt),
                body.eq(data.body),
                published.eq(data.published),
                posse.eq(data.posse),
                lang.eq(data.lang),
                updated_at.eq(now),
            ))
            .get_result::<Article>(conn)?;

        Ok(note)
    }
}

pub fn delete_article(article_id: i32, conn: &mut PgConnection) -> Result<usize> {
    use crate::schema::articles::dsl::*;
    use crate::schema::mentions;

    let num_deleted = conn.transaction(move |conn| {
        diesel::delete(mentions::table.filter(mentions::article_id.eq(article_id))).execute(conn)?;
        diesel::delete(articles.filter(id.eq(article_id))).execute(conn)
    })?;

    Ok(num_deleted)
}

pub fn get_articles_for_year_and_month(
    year: i32,
    month: u32,
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Article>> {
    use crate::schema::articles::dsl::*;

    let date = NaiveDate::from_ymd(year, month, 1);
    let time = NaiveTime::from_hms(0, 0, 0);
    let dt = NaiveDateTime::new(date, time);
    let days_in_mon = NaiveDate::from_ymd(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days();
    let dt_end = dt.checked_add_signed(Duration::days(days_in_mon)).unwrap_or(dt);

    let mut articles_list_query = articles
        .filter(inserted_at.gt(dt))
        .filter(inserted_at.lt(dt_end))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        articles_list_query = articles_list_query.filter(published.eq(only_visible));
    }

    let articles_list = articles_list_query.load::<Article>(conn)?;

    Ok(articles_list)
}

pub fn count_articles_for_year_and_month(
    year: i32,
    month: u32,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<i64> {
    use crate::schema::articles::dsl::*;
    use diesel::dsl::count;

    let date = NaiveDate::from_ymd(year, month, 1);
    let time = NaiveTime::from_hms(0, 0, 0);
    let dt = NaiveDateTime::new(date, time);
    let days_in_mon = NaiveDate::from_ymd(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days();
    let dt_end = dt.checked_add_signed(Duration::days(days_in_mon)).unwrap_or(dt);

    let mut articles_cnt_query = articles
        .select(count(id))
        .filter(inserted_at.gt(dt))
        .filter(inserted_at.lt(dt_end))
        .into_boxed();

    if only_visible {
        articles_cnt_query = articles_cnt_query.filter(published.eq(only_visible));
    }

    let articles_cnt = articles_cnt_query.first::<i64>(conn)?;

    Ok(articles_cnt)
}

pub fn get_articles_for_year(
    year: i32,
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Article>> {
    use crate::schema::articles::dsl::*;

    let date = NaiveDate::from_ymd(year, 1, 1);
    let time = NaiveTime::from_hms(0, 0, 0);
    let dt = NaiveDateTime::new(date, time);

    let date = NaiveDate::from_ymd(year, 12, 31);
    let time = NaiveTime::from_hms(23, 59, 59);
    let dt_end = NaiveDateTime::new(date, time);

    let mut articles_list_query = articles
        .filter(inserted_at.gt(dt))
        .filter(inserted_at.lt(dt_end))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        articles_list_query = articles_list_query.filter(published.eq(only_visible));
    }

    let articles_list = articles_list_query.load::<Article>(conn)?;

    Ok(articles_list)
}

pub fn count_articles_for_year(year: i32, only_visible: bool, conn: &mut PgConnection) -> Result<i64> {
    use crate::schema::articles::dsl::*;
    use diesel::dsl::count;

    let date = NaiveDate::from_ymd(year, 1, 1);
    let time = NaiveTime::from_hms(0, 0, 0);
    let dt = NaiveDateTime::new(date, time);

    let date = NaiveDate::from_ymd(year, 12, 31);
    let time = NaiveTime::from_hms(23, 59, 59);
    let dt_end = NaiveDateTime::new(date, time);

    let mut articles_cnt_query = articles
        .filter(inserted_at.gt(dt))
        .filter(inserted_at.lt(dt_end))
        .select(count(id))
        .into_boxed();

    if only_visible {
        articles_cnt_query = articles_cnt_query.filter(published.eq(only_visible));
    }

    let cnt = articles_cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}
