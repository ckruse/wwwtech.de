use chrono::NaiveDateTime;
use diesel::{dsl::exists, prelude::*, select};
use regex::Regex;
use std::str::FromStr;
use url::Url;

use crate::{
    models::{Mention, NewMention},
    DbError,
};

pub fn target_exists(url: &Url, conn: &PgConnection) -> Option<(bool, String, i32)> {
    use crate::schema::articles::dsl::{articles, id as art_id};
    use crate::schema::likes::dsl::{id as lik_id, likes};
    use crate::schema::notes::dsl::{id as not_id, notes};
    use crate::schema::pictures::dsl::{id as pic_id, pictures};

    let (object_type, obj_id) = match get_object_type_and_id(url) {
        Some((object_type, obj_id)) => (object_type, obj_id),
        None => return None,
    };

    let exists = match object_type.as_str() {
        "articles" => select(exists(articles.filter(art_id.eq(obj_id))))
            .get_result(conn)
            .unwrap_or_else(|_| false),
        "notes" => select(exists(notes.filter(not_id.eq(obj_id))))
            .get_result(conn)
            .unwrap_or_else(|_| false),
        "pictures" => select(exists(pictures.filter(pic_id.eq(obj_id))))
            .get_result(conn)
            .unwrap_or_else(|_| false),
        "likes" => select(exists(likes.filter(lik_id.eq(obj_id))))
            .get_result(conn)
            .unwrap_or_else(|_| false),
        _ => false,
    };

    Some((exists, object_type, obj_id))
}

pub fn get_object_type_and_id(url: &Url) -> Option<(String, i32)> {
    let path = url.path();
    let re = Regex::new(r"^/([^/]+)/(\d+)$").unwrap();

    let rslt = re.captures(path);

    if rslt.is_none() {
        return None;
    }

    let caps = rslt.unwrap();

    let object_type = caps.get(1);
    let id_str = caps.get(2);

    if object_type.is_none() || id_str.is_none() {
        return None;
    }

    let object_type = object_type.unwrap().as_str();
    let id: Result<i32, _> = FromStr::from_str(id_str.unwrap().as_str());

    if id.is_err() {
        return None;
    }

    Some((object_type.to_owned(), id.unwrap()))
}

pub fn mention_exists(source: &str, target: &str, conn: &PgConnection) -> bool {
    use crate::schema::mentions::dsl::*;

    select(exists(
        mentions.filter(target_url.eq(target)).filter(source_url.eq(source)),
    ))
    .get_result(conn)
    .unwrap_or_else(|_| false)
}

pub fn create_mention(
    source: &String,
    target: &String,
    object_type: &str,
    id: i32,
    conn: &PgConnection,
) -> Result<Mention, DbError> {
    use crate::schema::mentions;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = NewMention {
        source_url: source.clone(),
        target_url: target.clone(),
        author: "TODO:".to_owned(),
        mention_type: "TODO:".to_owned(),
        inserted_at: Some(now),
        updated_at: Some(now),
        ..Default::default()
    };

    match object_type {
        "note" => {
            data.note_id = Some(id);
        }
        "picture" => {
            data.picture_id = Some(id);
        }
        "article" => {
            data.article_id = Some(id);
        }
        _ => {}
    };

    let mention = diesel::insert_into(mentions::table)
        .values(data)
        .get_result::<Mention>(conn)?;

    Ok(mention)
}
