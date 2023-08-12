use chrono::Datelike;
use sqlx::{query, query_as, query_scalar, Connection, PgConnection};
use tokio::{fs::File, io::AsyncSeekExt};
use validator::Validate;

use crate::{
    models::{Deafie, NewDeafie},
    uri_helpers::root_uri,
    utils::{deafie_image_base_path, MONTHS},
};

pub async fn list_deafies(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Deafie>, sqlx::Error> {
    if only_visible {
        query_as!(
            Deafie,
            "SELECT * FROM deafies WHERE published = true ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT \
             $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    } else {
        query_as!(
            Deafie,
            "SELECT * FROM deafies ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    }
}

pub async fn count_deafies(only_visible: bool, conn: &mut PgConnection) -> Result<i64, sqlx::Error> {
    if only_visible {
        query_scalar("SELECT COUNT(*) FROM deafies WHERE published = true")
            .fetch_one(conn)
            .await
    } else {
        query_scalar("SELECT COUNT(*) FROM deafies").fetch_one(conn).await
    }
}

pub async fn get_deafie(deafie_id: i32, only_visible: bool, conn: &mut PgConnection) -> Result<Deafie, sqlx::Error> {
    if only_visible {
        query_as!(
            Deafie,
            "SELECT * FROM deafies WHERE id = $1 AND published = true",
            deafie_id
        )
        .fetch_one(conn)
        .await
    } else {
        query_as!(Deafie, "SELECT * FROM deafies WHERE id = $1", deafie_id)
            .fetch_one(conn)
            .await
    }
}

pub async fn get_deafie_by_slug(
    deafie_slug: &str,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Deafie, sqlx::Error> {
    if only_visible {
        query_as!(
            Deafie,
            "SELECT * FROM deafies WHERE slug = $1 AND published = true",
            deafie_slug
        )
        .fetch_one(conn)
        .await
    } else {
        query_as!(Deafie, "SELECT * FROM deafies WHERE slug = $1", deafie_slug)
            .fetch_one(conn)
            .await
    }
}

pub async fn get_youngest_deafie(only_visible: bool, conn: &mut PgConnection) -> Result<Deafie, sqlx::Error> {
    if only_visible {
        query_as!(
            Deafie,
            "SELECT * FROM deafies WHERE published = true ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT 1"
        )
        .fetch_one(conn)
        .await
    } else {
        query_as!(
            Deafie,
            "SELECT * FROM deafies ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT 1"
        )
        .fetch_one(conn)
        .await
    }
}

pub async fn create_deafie(
    data: &NewDeafie,
    file: Option<File>,
    conn: &mut PgConnection,
) -> Result<Deafie, Box<dyn std::error::Error + Send + Sync>> {
    let now = chrono::Utc::now().naive_utc();
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);

    let mon_idx = now.month0() as usize;
    let mut guid = String::new();
    guid.push_str(&now.year().to_string());
    guid.push('/');
    guid.push_str(MONTHS[mon_idx]);
    guid.push('/');
    guid.push_str(&data.slug);
    data.guid = Some(root_uri() + &guid.clone());
    data.slug = guid;

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    data.validate()?;

    let mut tx = conn.begin().await?;

    let deafie = query_as!(
        Deafie,
        r#"
        INSERT INTO deafies (
            author_id, title, slug, guid, image_name, image_content_type, excerpt,
            body, published, inserted_at, updated_at, posse_visibility, content_warning
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
        data.author_id,
        data.title,
        data.slug,
        data.guid,
        data.image_name,
        data.image_content_type,
        data.excerpt,
        data.body,
        data.published,
        data.inserted_at,
        data.updated_at,
        data.posse_visibility,
        data.content_warning
    )
    .fetch_one(&mut *tx)
    .await?;

    if let (Some(filename), Some(file)) = (&deafie.image_name, file) {
        let mut f = file.try_clone().await?;
        let path = format!("{}/{}/original", deafie_image_base_path(), deafie.id);
        tokio::fs::create_dir_all(path).await?;

        let path = format!("{}/{}/large", deafie_image_base_path(), deafie.id);
        tokio::fs::create_dir_all(path).await?;

        let path = format!("{}/{}/thumbnail", deafie_image_base_path(), deafie.id);
        tokio::fs::create_dir_all(path).await?;

        let path = format!("{}/{}/original/{}", deafie_image_base_path(), deafie.id, filename);

        let mut target_file = tokio::fs::File::create(path).await?;
        f.rewind().await?;
        tokio::io::copy(&mut f, &mut target_file).await?;
    }

    tx.commit().await?;

    Ok(deafie)
}

pub async fn update_deafie(
    deafie_id: i32,
    data: &NewDeafie,
    file: Option<File>,
    conn: &mut PgConnection,
) -> Result<Deafie, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = data.clone();

    if data.excerpt == Some("".to_owned()) {
        data.excerpt = None;
    }

    data.validate()?;

    let now = chrono::Utc::now().naive_utc();

    let deafie = query_as!(
        Deafie,
        r#"
            UPDATE deafies
            SET
                title = $1,
                slug = $2,
                excerpt = $3,
                body = $4,
                published = $5,
                updated_at = $6
            WHERE
                id = $7
            RETURNING *
        "#,
        data.title,
        data.slug,
        data.excerpt,
        data.body,
        data.published,
        now,
        deafie_id
    )
    .fetch_one(conn)
    .await?;

    if let (Some(file), Some(filename)) = (file, &deafie.image_name) {
        let mut f = file.try_clone().await?;
        let path = format!("{}/{}/original/{}", deafie_image_base_path(), deafie.id, filename);

        let mut target_file = File::create(path).await?;
        f.rewind().await?;
        tokio::io::copy(&mut f, &mut target_file).await?;
    }

    Ok(deafie)
}

pub async fn delete_deafie(deafie_id: i32, conn: &mut PgConnection) -> Result<Deafie, sqlx::Error> {
    let mut tx = conn.begin().await?;

    query!("DELETE FROM mentions WHERE deafie_id = $1", deafie_id)
        .execute(&mut *tx)
        .await?;

    let deafie = query_as!(Deafie, "DELETE FROM deafies WHERE id = $1 RETURNING *", deafie_id)
        .fetch_one(&mut *tx)
        .await?;

    let path = format!("{}/{}/", deafie_image_base_path(), deafie.id);
    // it doesn't matter when it fails
    let _rslt = tokio::fs::remove_dir_all(path).await;

    tx.commit().await?;

    Ok(deafie)
}
