use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::mentions;

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
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub article_id: Option<i32>,
    pub articles_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[table_name = "mentions"]
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

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
