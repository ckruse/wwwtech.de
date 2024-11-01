use anyhow::{Error, anyhow};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::{Connection, PgConnection, query, query_as, query_scalar};
use validator::Validate;

use crate::models::{Article, NewArticle};
use crate::uri_helpers::root_uri;
use crate::utils::MONTHS;

pub async fn list_articles(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Article>, sqlx::Error> {
    if only_visible {
        query_as!(
            Article,
            "SELECT * FROM articles WHERE published = true ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT \
             $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    } else {
        query_as!(
            Article,
            "SELECT * FROM articles ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    }
}

pub async fn count_articles(only_visible: bool, conn: &mut PgConnection) -> Result<i64, sqlx::Error> {
    if only_visible {
        query_scalar("SELECT COUNT(*) FROM articles WHERE published = true")
            .fetch_one(conn)
            .await
    } else {
        query_scalar("SELECT COUNT(*) FROM articles").fetch_one(conn).await
    }
}

pub async fn get_youngest_article(only_visible: bool, conn: &mut PgConnection) -> Result<Article, sqlx::Error> {
    if only_visible {
        query_as!(
            Article,
            "SELECT * FROM articles WHERE published = true ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT 1"
        )
        .fetch_one(conn)
        .await
    } else {
        query_as!(Article, "SELECT * FROM articles ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT 1")
            .fetch_one(conn)
            .await
    }
}

pub async fn get_article(article_id: i32, only_visible: bool, conn: &mut PgConnection) -> Result<Article, sqlx::Error> {
    if only_visible {
        query_as!(Article, "SELECT * FROM articles WHERE id = $1 AND published = true", article_id)
            .fetch_one(conn)
            .await
    } else {
        query_as!(Article, "SELECT * FROM articles WHERE id = $1", article_id)
            .fetch_one(conn)
            .await
    }
}

pub async fn get_article_by_slug(
    article_slug: &str,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Option<Article>, sqlx::Error> {
    if only_visible {
        query_as!(Article, "SELECT * FROM articles WHERE slug = $1 AND published = true", article_slug)
            .fetch_optional(conn)
            .await
    } else {
        query_as!(Article, "SELECT * FROM articles WHERE slug = $1", article_slug)
            .fetch_optional(conn)
            .await
    }
}

pub async fn get_article_by_slug_part(
    article_slug: &str,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Option<Article>, sqlx::Error> {
    let article_slug = format!("%/{}", article_slug);

    if only_visible {
        query_as!(Article, "SELECT * FROM articles WHERE slug ILIKE $1 AND published = true", article_slug)
            .fetch_optional(conn)
            .await
    } else {
        query_as!(Article, "SELECT * FROM articles WHERE slug ILIKE $1", article_slug)
            .fetch_optional(conn)
            .await
    }
}

pub async fn create_article(
    data: &NewArticle,
    conn: &mut PgConnection,
) -> Result<Article, Box<dyn std::error::Error + Send + Sync>> {
    let now = chrono::Utc::now().naive_utc();

    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);
    data.article_format = Some("markdown".to_owned());

    let mon_idx = now.month0() as usize;
    let mut guid = String::new();
    guid.push_str(&now.year().to_string());
    guid.push('/');
    guid.push_str(MONTHS[mon_idx]);
    guid.push('/');
    guid.push_str(&data.slug);
    data.guid = Some(root_uri() + &guid.clone());
    data.slug = guid;

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    data.validate()?;

    let article = query_as!(
        Article,
        r#"
            INSERT INTO articles (
                author_id, in_reply_to, title, slug, guid, article_format, excerpt, body, published, posse,
                lang, inserted_at, updated_at, posse_visibility, content_warning
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            )
            RETURNING *
        "#,
        data.author_id,
        data.in_reply_to,
        data.title,
        data.slug,
        data.guid,
        data.article_format,
        data.excerpt,
        data.body,
        data.published,
        data.posse,
        data.lang,
        data.inserted_at,
        data.updated_at,
        data.posse_visibility,
        data.content_warning
    )
    .fetch_one(conn)
    .await?;

    Ok(article)
}

pub async fn update_article(
    article_id: i32,
    data: &NewArticle,
    conn: &mut PgConnection,
) -> Result<Article, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = data.clone();

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    data.validate()?;

    let now = chrono::Utc::now().naive_utc();

    let note = query_as!(
        Article,
        r#"
            UPDATE articles
            SET
                in_reply_to = $1,
                title = $2,
                slug = $3,
                excerpt = $4,
                body = $5,
                published = $6,
                posse = $7,
                lang = $8,
                updated_at = $9
            WHERE id = $10
            RETURNING *
        "#,
        data.in_reply_to,
        data.title,
        data.slug,
        data.excerpt,
        data.body,
        data.published,
        data.posse,
        data.lang,
        now,
        article_id
    )
    .fetch_one(conn)
    .await?;

    Ok(note)
}

pub async fn delete_article(article_id: i32, conn: &mut PgConnection) -> Result<Article, sqlx::Error> {
    let mut tx = conn.begin().await?;

    query!("DELETE FROM mentions WHERE article_id = $1", article_id)
        .execute(&mut *tx)
        .await?;

    let article = query_as!(Article, "DELETE FROM articles WHERE id = $1 RETURNING *", article_id)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(article)
}

pub async fn get_articles_for_year_and_month(
    year: i32,
    month: u32,
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Article>, Error> {
    let date = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| anyhow!("invalid date"))?;
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(date, time);
    let days_in_mon =
        NaiveDate::from_ymd_opt(if month == 12 { year + 1 } else { year }, if month == 12 { 1 } else { month + 1 }, 1)
            .ok_or_else(|| anyhow!("invalid date"))?
            .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| anyhow!("invalid_date"))?)
            .num_days();
    let dt_end = dt.checked_add_signed(Duration::days(days_in_mon)).unwrap_or(dt);

    let articles = if only_visible {
        query_as!(
            Article,
            "SELECT * FROM articles WHERE inserted_at > $1 AND inserted_at < $2 ORDER BY inserted_at DESC, updated_at \
             DESC, id DESC LIMIT $3 OFFSET $4",
            dt,
            dt_end,
            limit,
            offset
        )
        .fetch_all(conn)
        .await?
    } else {
        query_as!(
            Article,
            "SELECT * FROM articles WHERE inserted_at > $1 AND inserted_at < $2 ORDER BY inserted_at DESC, updated_at \
             DESC, id DESC LIMIT $3 OFFSET $4",
            dt,
            dt_end,
            limit,
            offset
        )
        .fetch_all(conn)
        .await?
    };

    Ok(articles)
}

pub async fn count_articles_for_year_and_month(
    year: i32,
    month: u32,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<i64, Error> {
    let date = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| anyhow!("invalid date"))?;
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(date, time);
    let days_in_mon =
        NaiveDate::from_ymd_opt(if month == 12 { year + 1 } else { year }, if month == 12 { 1 } else { month + 1 }, 1)
            .ok_or_else(|| anyhow!("invalid date"))?
            .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| anyhow!("invalid_date"))?)
            .num_days();
    let dt_end = dt.checked_add_signed(Duration::days(days_in_mon)).unwrap_or(dt);

    let cnt = if only_visible {
        query_scalar!("SELECT COUNT(*) FROM articles WHERE inserted_at > $1 AND inserted_at < $2", dt, dt_end)
            .fetch_one(conn)
            .await?
    } else {
        query_scalar!("SELECT COUNT(*) FROM articles WHERE inserted_at > $1 AND inserted_at < $2", dt, dt_end)
            .fetch_one(conn)
            .await?
    };

    Ok(cnt.unwrap_or(0))
}

pub async fn get_articles_for_year(
    year: i32,
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Article>, Error> {
    let date = NaiveDate::from_ymd_opt(year, 1, 1).ok_or_else(|| anyhow!("invalid date"))?;
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(date, time);

    let date = NaiveDate::from_ymd_opt(year, 12, 31).ok_or_else(|| anyhow!("invalid date"))?;
    let time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let dt_end = NaiveDateTime::new(date, time);

    let articles = if only_visible {
        query_as!(
            Article,
            "SELECT * FROM articles WHERE inserted_at > $1 AND inserted_at < $2 ORDER BY inserted_at DESC, updated_at \
             DESC, id DESC LIMIT $3 OFFSET $4",
            dt,
            dt_end,
            limit,
            offset
        )
        .fetch_all(conn)
        .await?
    } else {
        query_as!(
            Article,
            "SELECT * FROM articles WHERE inserted_at > $1 AND inserted_at < $2 ORDER BY inserted_at DESC, updated_at \
             DESC, id DESC LIMIT $3 OFFSET $4",
            dt,
            dt_end,
            limit,
            offset
        )
        .fetch_all(conn)
        .await?
    };

    Ok(articles)
}

pub async fn count_articles_for_year(year: i32, only_visible: bool, conn: &mut PgConnection) -> Result<i64, Error> {
    let date = NaiveDate::from_ymd_opt(year, 1, 1).ok_or_else(|| anyhow!("invalid date"))?;
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(date, time);

    let date = NaiveDate::from_ymd_opt(year, 12, 31).ok_or_else(|| anyhow!("invalid date"))?;
    let time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let dt_end = NaiveDateTime::new(date, time);

    let cnt = if only_visible {
        query_scalar!("SELECT COUNT(*) FROM articles WHERE inserted_at > $1 AND inserted_at < $2", dt, dt_end)
            .fetch_one(conn)
            .await?
    } else {
        query_scalar!("SELECT COUNT(*) FROM articles WHERE inserted_at > $1 AND inserted_at < $2", dt, dt_end)
            .fetch_one(conn)
            .await?
    };

    Ok(cnt.unwrap_or(0))
}
