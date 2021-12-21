use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::authors;

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
