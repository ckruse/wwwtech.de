use async_trait::async_trait;
use axum_login::AuthnBackend;
use serde::Deserialize;
use sqlx::{query_as, PgPool};

use crate::models::Author;

#[derive(Clone, Debug)]
pub struct Store {
    pool: PgPool,
}

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for Store {
    type User = Author;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let user = query_as!(Author, "SELECT * FROM authors WHERE email = $1", creds.email)
            .fetch_optional(&self.pool)
            .await?;

        let Some(user) = user else {
            return Ok(None);
        };

        if crate::session::actions::verify_password(&user, &creds.password) {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &i32) -> Result<Option<Self::User>, sqlx::Error> {
        let user = query_as!(Author, "SELECT * FROM authors WHERE id = $1", user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
