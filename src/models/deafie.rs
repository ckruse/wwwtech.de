use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use validator::{Validate, ValidationError};

use crate::schema::deafies;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Deafie {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub guid: String,
    pub excerpt: Option<String>,
    pub body: String,
    pub published: bool,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
