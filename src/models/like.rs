use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Like {
    pub id: i32,
    pub in_reply_to: String,

    pub author_id: i32,

    pub posse: bool,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub show_in_index: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
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
