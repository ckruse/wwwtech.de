use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::schema::{articles, authors, likes, mentions, notes, pictures};

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: String,
    pub encrypted_password: String,
    pub remember_created_at: Option<NaiveDateTime>,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub in_reply_to: Option<String>,

    pub title: String,
    pub slug: String,
    pub guid: String,
    pub article_format: String,

    pub excerpt: Option<String>,
    pub body: String,

    pub published: bool,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub posse: bool,
    pub lang: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub author_id: i32,
    pub content: String,
    pub in_reply_to: Option<String>,
    pub webmentions_count: i32,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub posse: bool,
    pub title: String,
    pub show_in_index: bool,
    pub lang: String,
    pub note_type: String,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[table_name = "notes"]
pub struct NewNote {
    pub author_id: Option<i32>,
    #[validate(length(min = 5))]
    pub title: String,
    #[validate(custom = "validate_note_type")]
    pub note_type: String,
    #[validate(url)]
    pub in_reply_to: Option<String>,
    #[validate(length(min = 2, max = 2))]
    pub lang: String,
    #[serde(default)]
    pub posse: bool,
    #[serde(default)]
    pub show_in_index: bool,
    #[validate(required, length(min = 5))]
    pub content: Option<String>,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

fn validate_note_type(note_type: &str) -> Result<(), ValidationError> {
    if note_type != "note" && note_type != "reply" && note_type != "repost" {
        return Err(ValidationError::new("note_type is invalid"));
    }

    Ok(())
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Picture {
    pub id: i32,
    pub author_id: i32,

    pub in_reply_to: Option<String>,
    pub webmentions_count: i32,

    pub image_file_name: String,
    pub image_content_type: String,
    pub image_file_size: i32,
    pub image_updated_at: NaiveDateTime,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub title: String,

    pub posse: bool,
    pub show_in_index: bool,

    pub content: String,

    pub lang: String,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Like {
    pub id: i32,
    pub in_reply_to: String,

    pub author_id: i32,

    pub posse: bool,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub show_in_index: bool,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[table_name = "likes"]
pub struct NewLike {
    pub author_id: Option<i32>,
    #[validate(url, length(min = 3))]
    pub in_reply_to: String,
    #[serde(default)]
    pub posse: bool,
    #[serde(default)]
    pub show_in_index: bool,
    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Mention {
    pub id: i32,
    pub source_url: String,
    pub target_url: String,

    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub author: String,
    pub author_url: Option<String>,
    pub author_avatar: Option<String>,

    pub mention_type: String,

    pub note_id: Option<i32>,
    pub picture_id: Option<i32>,
    pub article_id: Option<i32>,
    pub articles_id: Option<i32>,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
