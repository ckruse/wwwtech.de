use std::vec::Vec;

use sqlx::{Connection, PgConnection, query, query_as, query_scalar};
use validator::Validate;

use crate::models::{NewNote, Note};

pub async fn list_notes(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Note>, sqlx::Error> {
    if only_visible {
        query_as!(
            Note,
            "SELECT * FROM notes WHERE show_in_index = $1 ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT \
             $2 OFFSET $3",
            only_visible,
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    } else {
        query_as!(
            Note,
            "SELECT * FROM notes ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    }
}

pub async fn count_notes(only_visible: bool, conn: &mut PgConnection) -> Result<i64, sqlx::Error> {
    if only_visible {
        query_scalar("SELECT COUNT(*) FROM notes WHERE show_in_index = $1")
            .bind(only_visible)
            .fetch_one(conn)
            .await
    } else {
        query_scalar("SELECT COUNT(*) FROM notes").fetch_one(conn).await
    }
}

pub async fn get_note(note_id: i32, conn: &mut PgConnection) -> Result<Note, sqlx::Error> {
    query_as!(Note, "SELECT * FROM notes WHERE id = $1", note_id)
        .fetch_one(conn)
        .await
}

pub async fn create_note(
    data: &NewNote,
    conn: &mut PgConnection,
) -> Result<Note, Box<dyn std::error::Error + Send + Sync>> {
    let now = chrono::Utc::now().naive_utc();
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let note = query_as!(
            Note,
            r#"
            INSERT INTO notes (author_id, title, note_type, in_reply_to, lang, posse, show_in_index, content, inserted_at, updated_at, posse_visibility, content_warning)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
            data.author_id,
            data.title,
            data.note_type,
            data.in_reply_to,
            data.lang,
            data.posse,
            data.show_in_index,
            data.content,
            data.inserted_at,
            data.updated_at,
            data.posse_visibility,
            data.content_warning
        )
        .fetch_one(conn)
        .await?;

        Ok(note)
    }
}

pub async fn update_note(
    note_id: i32,
    data: &NewNote,
    conn: &mut PgConnection,
) -> Result<Note, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = data.clone();

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    if let Err(errors) = data.validate() {
        Err(Box::new(errors))
    } else {
        let now = chrono::Utc::now().naive_local();

        let note = query_as!(
            Note,
            r#"
            UPDATE notes
            SET title = $1, lang = $2, in_reply_to = $3, posse = $4, show_in_index = $5, content = $6, updated_at = $7
            WHERE id = $8
            RETURNING *
            "#,
            data.title,
            data.lang,
            data.in_reply_to,
            data.posse,
            data.show_in_index,
            data.content,
            now,
            note_id
        )
        .fetch_one(conn)
        .await?;

        Ok(note)
    }
}

pub async fn delete_note(note_id: i32, conn: &mut PgConnection) -> Result<Note, sqlx::Error> {
    let mut tx = conn.begin().await?;

    query!("DELETE FROM mentions WHERE note_id = $1", note_id)
        .execute(&mut *tx)
        .await?;
    let note = query_as!(Note, "DELETE FROM notes WHERE id = $1 RETURNING *", note_id)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(note)
}
