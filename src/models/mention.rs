use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub article_id: Option<i32>,
    pub articles_id: Option<i32>,
    pub deafie_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct NewMention {
    #[validate(url, length(min = 3))]
    pub source_url: String,
    #[validate(url, length(min = 3))]
    pub target_url: String,

    pub title: String,
    pub author: String,
    pub mention_type: String,

    pub note_id: Option<i32>,
    pub picture_id: Option<i32>,
    pub article_id: Option<i32>,
    pub deafie_id: Option<i32>,

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
