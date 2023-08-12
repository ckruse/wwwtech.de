use chrono::Utc;
use sqlx::{query, query_as, query_scalar, Connection, PgConnection};
use tokio::{fs::File, io::AsyncSeekExt};
use validator::Validate;

use crate::{
    models::{NewPicture, Picture},
    utils::image_base_path,
};

pub async fn list_pictures(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Picture>, sqlx::Error> {
    if only_visible {
        query_as!(
            Picture,
            "SELECT * FROM pictures WHERE show_in_index = $1 ORDER BY inserted_at DESC, updated_at DESC, id DESC \
             LIMIT $2 OFFSET $3",
            only_visible,
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    } else {
        query_as!(
            Picture,
            "SELECT * FROM pictures ORDER BY inserted_at DESC, updated_at DESC, id DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(conn)
        .await
    }
}

pub async fn count_pictures(only_visible: bool, conn: &mut PgConnection) -> Result<i64, sqlx::Error> {
    if only_visible {
        query_scalar("SELECT COUNT(*) FROM pictures WHERE show_in_index = $1")
            .bind(only_visible)
            .fetch_one(conn)
            .await
    } else {
        query_scalar("SELECT COUNT(*) FROM pictures").fetch_one(conn).await
    }
}

pub async fn get_picture(picture_id: i32, conn: &mut PgConnection) -> Result<Picture, sqlx::Error> {
    query_as!(Picture, "SELECT * FROM pictures WHERE id = $1", picture_id)
        .fetch_one(conn)
        .await
}

pub async fn create_picture(
    data: &NewPicture,
    file: Option<File>,
    conn: &mut PgConnection,
) -> Result<Picture, Box<dyn std::error::Error + Send + Sync>> {
    let Some(mut file) = file else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no picture given",
        )));
    };

    let now = Utc::now().naive_utc();

    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);
    data.image_updated_at = Some(now);

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.alt == Some("".to_owned()) {
        data.alt = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    data.validate()?;

    let mut tx = conn.begin().await?;
    let picture = query_as!(
        Picture,
        r#"
            INSERT INTO pictures
                (author_id, in_reply_to, image_file_name, image_content_type, image_file_size, image_updated_at,
                 inserted_at, updated_at, title, posse, show_in_index, content, lang, alt, posse_visibility,
                 content_warning)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
        "#,
        data.author_id,
        data.in_reply_to,
        data.image_file_name,
        data.image_content_type,
        0,
        data.image_updated_at,
        data.inserted_at,
        data.updated_at,
        data.title,
        data.posse,
        data.show_in_index,
        data.content,
        data.lang,
        data.alt,
        data.posse_visibility,
        data.content_warning
    )
    .fetch_one(&mut *tx)
    .await?;

    let path = format!("{}/{}/original", image_base_path(), picture.id);
    std::fs::create_dir_all(path)?;

    let path = format!("{}/{}/large", image_base_path(), picture.id);
    std::fs::create_dir_all(path)?;

    let path = format!("{}/{}/thumbnail", image_base_path(), picture.id);
    std::fs::create_dir_all(path)?;

    let path = format!(
        "{}/{}/original/{}",
        image_base_path(),
        picture.id,
        picture.image_file_name
    );

    let mut target_file = File::create(path).await?;
    file.rewind().await?;
    tokio::io::copy(&mut file, &mut target_file).await?;

    tx.commit().await?;

    Ok(picture)
}

pub async fn update_picture(
    picture: &Picture,
    data: &NewPicture,
    file: Option<File>,
    conn: &mut PgConnection,
) -> Result<Picture, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = data.clone();
    let picture = picture.clone();

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.alt == Some("".to_owned()) {
        data.alt = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    data.validate()?;

    let now = Utc::now().naive_utc();

    let picture = query_as!(
        Picture,
        r#"
            UPDATE pictures
            SET
              in_reply_to = $1,
              image_file_name = $2,
              image_content_type = $3,
              image_updated_at = $4,
              updated_at = $5,
              title = $6,
              posse = $7,
              show_in_index = $8,
              content = $9,
              lang = $10,
              alt = $11,
              posse_visibility = $12,
              content_warning = $13
            WHERE id = $14
            RETURNING *
        "#,
        data.in_reply_to.or(picture.in_reply_to),
        data.image_file_name.unwrap_or(picture.image_file_name),
        data.image_content_type.unwrap_or(picture.image_content_type),
        now,
        now,
        data.title,
        data.posse,
        data.show_in_index,
        data.content.unwrap_or(picture.content),
        data.lang,
        data.alt,
        data.posse_visibility,
        data.content_warning.or(picture.content_warning),
        picture.id
    )
    .fetch_one(conn)
    .await?;

    if let Some(mut file) = file {
        let path = format!(
            "{}/{}/original/{}",
            image_base_path(),
            picture.id,
            picture.image_file_name
        );

        let mut target_file = File::create(path).await?;
        file.rewind().await?;
        tokio::io::copy(&mut file, &mut target_file).await?;
    }

    Ok(picture)
}

pub async fn delete_picture(picture: &Picture, conn: &mut PgConnection) -> Result<Picture, sqlx::Error> {
    let mut tx = conn.begin().await?;

    query!("DELETE FROM mentions WHERE picture_id = $1", picture.id)
        .execute(&mut *tx)
        .await?;

    let picture = query_as!(Picture, "DELETE FROM pictures WHERE id = $1 RETURNING *", picture.id)
        .fetch_one(&mut *tx)
        .await?;
    let path = format!("{}/{}/", image_base_path(), picture.id);
    // it doesn't matter when it fails
    let _rslt = std::fs::remove_dir_all(path);

    tx.commit().await?;

    Ok(picture)
}
