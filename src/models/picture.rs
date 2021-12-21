use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use anyhow::Result;
use background_jobs::Job;
use image::imageops::FilterType;
use image::GenericImageView;
use std::future::Future;
use std::pin::Pin;

use crate::schema::pictures;
use crate::utils::image_base_path;
use crate::DEFAULT_QUEUE;

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

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[table_name = "pictures"]
pub struct NewPicture {
    pub author_id: Option<i32>,
    #[validate(length(min = 5))]
    pub title: String,
    #[validate(length(min = 5))]
    pub alt: Option<String>,
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

    pub image_file_name: Option<String>,
    pub image_content_type: Option<String>,
    pub image_file_size: Option<i32>,
    pub image_updated_at: Option<NaiveDateTime>,

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
