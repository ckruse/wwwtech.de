use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::deafies;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Deafie {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub guid: String,
    pub image_name: Option<String>,
    pub image_content_type: Option<String>,
    pub excerpt: Option<String>,
    pub body: String,
    pub published: bool,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[table_name = "deafies"]
pub struct NewDeafie {
    pub author_id: Option<i32>,
    #[validate(length(min = 3, max = 255))]
    pub title: String,
    #[validate(length(min = 3, max = 255))]
    pub slug: String,
    pub guid: Option<String>,
    pub excerpt: Option<String>,
    #[validate(length(min = 3))]
    pub body: String,

    #[serde(default)]
    pub published: bool,

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
