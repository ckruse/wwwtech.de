use std::vec::Vec;

use sqlx::{PgConnection, query_as, query_scalar};
use validator::Validate;

use crate::models::{Like, NewLike};

pub async fn list_likes(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Like>, sqlx::Error> {
    if only_visible {
        query_as!(
            Like,
            "SELECT * FROM likes WHERE show_in_index = $1 ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT \
             $2 OFFSET $3",
            only_visible,
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    } else {
        query_as!(
            Like,
            "SELECT * FROM likes ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    }
}

pub async fn count_likes(only_visible: bool, conn: &mut PgConnection) -> Result<i64, sqlx::Error> {
    if only_visible {
        query_scalar("SELECT COUNT(*) FROM likes WHERE show_in_index = $1")
            .bind(only_visible)
            .fetch_one(conn)
            .await
    } else {
        query_scalar("SELECT COUNT(*) FROM likes").fetch_one(conn).await
    }
}

pub async fn get_like(like_id: i32, conn: &mut PgConnection) -> Result<Like, sqlx::Error> {
    query_as!(Like, "SELECT * FROM likes WHERE id = $1", like_id)
        .fetch_one(conn)
        .await
}

pub async fn create_like(
    data: &NewLike,
    conn: &mut PgConnection,
) -> Result<Like, Box<dyn std::error::Error + Send + Sync>> {
    let now = chrono::Utc::now().naive_utc();

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let like = query_as!(
            Like,
            "INSERT INTO likes (in_reply_to, posse, show_in_index, inserted_at, updated_at) VALUES ($1, $2, $3, $4, \
             $5) RETURNING *",
            data.in_reply_to,
            data.posse,
            data.show_in_index,
            now,
            now
        )
        .fetch_one(conn)
        .await?;

        Ok(like)
    }
}

pub async fn update_like(
    like_id: i32,
    data: &NewLike,
    conn: &mut PgConnection,
) -> Result<Like, Box<dyn std::error::Error + Send + Sync>> {
    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let now = chrono::Utc::now().naive_utc();
        let like = query_as!(
            Like,
            "UPDATE likes SET in_reply_to = $1, posse = $2, show_in_index = $3, updated_at = $4 WHERE id = $5 \
             RETURNING *",
            data.in_reply_to,
            data.posse,
            data.show_in_index,
            now,
            like_id
        )
        .fetch_one(conn)
        .await?;

        Ok(like)
    }
}

pub async fn delete_like(like_id: i32, conn: &mut PgConnection) -> Result<Like, sqlx::Error> {
    query_as!(Like, "DELETE FROM likes WHERE id = $1 RETURNING *", like_id)
        .fetch_one(&mut *conn)
        .await
}
