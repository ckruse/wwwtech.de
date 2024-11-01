use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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

    pub posse_visibility: String,
    pub content_warning: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct NewArticle {
    pub author_id: Option<i32>,

    pub in_reply_to: Option<String>,

    #[validate(length(min = 3, max = 255))]
    pub title: String,
    #[validate(length(min = 3, max = 255))]
    pub slug: String,
    pub guid: Option<String>,
    pub article_format: Option<String>,

    pub excerpt: Option<String>,
    #[validate(length(min = 3))]
    pub body: String,

    #[serde(default)]
    pub published: bool,
    #[serde(default)]
    pub posse: bool,
    #[validate(length(min = 2, max = 2))]
    pub lang: String,

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,

    pub posse_visibility: String,
    pub content_warning: Option<String>,
}
