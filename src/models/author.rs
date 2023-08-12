use axum_login::{secrecy::SecretVec, AuthUser};
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

impl AuthUser<i32> for Author {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.encrypted_password.clone().into())
    }
}
