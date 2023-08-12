use async_trait::async_trait;
use axum_login::UserStore;
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

#[async_trait]
impl UserStore<i32, ()> for Store {
    type User = Author;

    async fn load_user(&self, user_id: &i32) -> Result<Option<Self::User>, eyre::Error> {
        let user = query_as!(Author, "SELECT * FROM authors WHERE id = $1", user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
