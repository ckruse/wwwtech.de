use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub posse_visibility: String,
    pub content_warning: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, Validate)]
pub struct NewNote {
    pub author_id: Option<i32>,
    #[validate(length(min = 5))]
    pub title: String,
    #[validate(custom(function = "validate_note_type"))]
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
    pub posse_visibility: String,
    pub content_warning: Option<String>,
}

fn validate_note_type(note_type: &str) -> Result<(), ValidationError> {
    if note_type != "note" && note_type != "reply" && note_type != "repost" {
        return Err(ValidationError::new("note_type is invalid"));
    }

    Ok(())
}
