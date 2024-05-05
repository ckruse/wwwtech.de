use std::str::FromStr;

use sqlx::{query_as, query_scalar, PgConnection};
use url::Url;

use crate::models::{Mention, NewMention};

pub async fn target_exists(url: &Url, conn: &mut PgConnection) -> Option<(ObjectType, i32)> {
    let (object_type, obj_id) = match get_object_type_and_id(url) {
        Some((object_type, obj_id)) => (object_type, obj_id),
        None => return None,
    };

    let exists: bool = match object_type {
        ObjectType::Article => query_scalar!("SELECT EXISTS(SELECT true FROM articles WHERE id = $1)", obj_id)
            .fetch_one(conn)
            .await
            .ok()
            .flatten()
            .unwrap_or(false),
        ObjectType::Deafie => query_scalar!("SELECT EXISTS(SELECT true FROM deafies WHERE id = $1)", obj_id)
            .fetch_one(conn)
            .await
            .ok()
            .flatten()
            .unwrap_or(false),
        ObjectType::Note => query_scalar!("SELECT EXISTS(SELECT true FROM notes WHERE id = $1)", obj_id)
            .fetch_one(conn)
            .await
            .ok()
            .flatten()
            .unwrap_or(false),
        ObjectType::Picture => query_scalar!("SELECT EXISTS(SELECT 1 FROM pictures WHERE id = $1)", obj_id)
            .fetch_one(conn)
            .await
            .ok()
            .flatten()
            .unwrap_or(false),
        ObjectType::Like => query_scalar!("SELECT EXISTS(SELECT 1 FROM likes WHERE id = $1)", obj_id)
            .fetch_one(conn)
            .await
            .ok()
            .flatten()
            .unwrap_or(false),
    };

    if exists { Some((object_type, obj_id)) } else { None }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectType {
    Article,
    Note,
    Picture,
    Like,
    Deafie,
}

impl FromStr for ObjectType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "articles" => Ok(ObjectType::Article),
            "notes" => Ok(ObjectType::Note),
            "pictures" => Ok(ObjectType::Picture),
            "likes" => Ok(ObjectType::Like),
            "deafies" => Ok(ObjectType::Deafie),
            _ => Err(()),
        }
    }
}

pub fn get_object_type_and_id(url: &Url) -> Option<(ObjectType, i32)> {
    let path = url.path();
    let pieces = path.split('/').collect::<Vec<_>>();

    let object_type = pieces.get(pieces.len() - 2)?;
    let id_str = pieces.last()?;

    let object_type = ObjectType::from_str(object_type).ok()?;
    let id: i32 = FromStr::from_str(id_str).ok()?;

    Some((object_type, id))
}

pub async fn mention_exists(source: &str, target: &str, conn: &mut PgConnection) -> bool {
    query_scalar!(
        "SELECT EXISTS(SELECT true FROM mentions WHERE source_url = $1 AND target_url = $2)",
        source,
        target
    )
    .fetch_one(conn)
    .await
    .ok()
    .flatten()
    .unwrap_or(false)
}

pub async fn create_mention(
    source_url: String,
    target_url: String,
    object_type: ObjectType,
    id: i32,
    author: String,
    title: String,
    conn: &mut PgConnection,
) -> Result<Mention, sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();
    let mut data = NewMention {
        source_url,
        target_url,
        author,
        title,
        mention_type: "TODO:".to_owned(),
        inserted_at: Some(now),
        updated_at: Some(now),
        ..Default::default()
    };

    match object_type {
        ObjectType::Note => {
            data.note_id = Some(id);
        }
        ObjectType::Picture => {
            data.picture_id = Some(id);
        }
        ObjectType::Article => {
            data.article_id = Some(id);
        }
        ObjectType::Deafie => {
            data.article_id = Some(id);
        }
        _ => {}
    };

    query_as!(
        Mention,
        r#"
        INSERT INTO mentions (source_url, target_url, author, title, mention_type, inserted_at, updated_at, note_id, picture_id, article_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
        data.source_url,
        data.target_url,
        data.author,
        data.title,
        data.mention_type,
        data.inserted_at,
        data.updated_at,
        data.note_id,
        data.picture_id,
        data.article_id
    )
    .fetch_one(conn)
    .await
}
