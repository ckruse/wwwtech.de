use chrono::{Datelike, NaiveDateTime};
use diesel::prelude::*;
use std::vec::Vec;
use validator::Validate;

use crate::models::{Article, NewArticle};
use crate::uri_helpers::root_uri;
use crate::DbError;

pub fn list_articles(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &PgConnection,
) -> Result<Vec<Article>, DbError> {
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

pub fn count_articles(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::articles::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = articles.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(published.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_youngest_article(only_visible: bool, conn: &PgConnection) -> Result<Article, DbError> {
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

pub fn get_article(article_id: i32, only_visible: bool, conn: &PgConnection) -> Result<Article, DbError> {
    use crate::schema::articles::dsl::*;

    let mut article_query = articles.filter(id.eq(article_id)).into_boxed();

    if only_visible {
        article_query = article_query.filter(published.eq(only_visible));
    }

    let article = article_query.first::<Article>(conn)?;

    Ok(article)
}

pub fn get_article_by_slug(article_slug: &str, only_visible: bool, conn: &PgConnection) -> Result<Article, DbError> {
    use crate::schema::articles::dsl::*;

    let mut article_query = articles.filter(slug.eq(article_slug)).into_boxed();

    if only_visible {
        article_query = article_query.filter(published.eq(only_visible));
    }

    let article = article_query.first::<Article>(conn)?;

    Ok(article)
}

static MONTHS: [&'static str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

pub fn create_article(data: &NewArticle, conn: &PgConnection) -> Result<Article, DbError> {
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
        Err(Box::new(errors))
    } else {
        let article = diesel::insert_into(articles::table)
            .values(data)
            .get_result::<Article>(conn)?;

        Ok(article)
    }
}

pub fn update_article(article_id: i32, data: &NewArticle, conn: &PgConnection) -> Result<Article, DbError> {
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
        Err(Box::new(errors))
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

pub fn delete_article(article_id: i32, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::articles::dsl::*;
    use crate::schema::mentions;

    let num_deleted = conn.transaction(move || {
        diesel::delete(mentions::table.filter(mentions::article_id.eq(article_id))).execute(conn)?;
        diesel::delete(articles.filter(id.eq(article_id))).execute(conn)
    })?;

    Ok(num_deleted)
}
