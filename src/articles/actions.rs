use diesel::prelude::*;
use std::vec::Vec;

use crate::models::Article;
use crate::DbError;

pub fn list_articles(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &PgConnection,
) -> Result<Vec<Article>, DbError> {
    use crate::schema::articles::dsl::*;

    let articles_list = articles
        .filter(published.eq(only_visible))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .load::<Article>(conn)?;

    Ok(articles_list)
}

pub fn count_articles(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::articles::dsl::*;
    use diesel::dsl::count;

    let cnt = articles
        .filter(published.eq(only_visible))
        .select(count(id))
        .first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_youngest_article(only_visible: bool, conn: &PgConnection) -> Result<Article, DbError> {
    use crate::schema::articles::dsl::*;
    let article = articles
        .filter(published.eq(only_visible))
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(1)
        .first::<Article>(conn)?;

    Ok(article)
}

pub fn get_article(article_id: i32, only_visible: bool, conn: &PgConnection) -> Result<Article, DbError> {
    use crate::schema::articles::dsl::*;

    let article = articles
        .filter(published.eq(only_visible))
        .filter(id.eq(article_id))
        .first::<Article>(conn)?;

    Ok(article)
}
