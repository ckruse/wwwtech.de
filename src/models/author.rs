use axum_login::AuthUser;
use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl AuthUser for Author {
    type Id = i32;

    fn id(&self) -> i32 {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.encrypted_password.as_bytes()
    }
}
