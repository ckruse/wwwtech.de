use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;
use sqlx::{query_as, PgConnection};

use crate::{errors::AppError, models::Author};

pub async fn get_author_by_email(user_email: &str, conn: &mut PgConnection) -> Result<Author, AppError> {
    let author = query_as!(Author, "SELECT * FROM authors WHERE email = $1", user_email)
        .fetch_one(conn)
        .await?;

    Ok(author)
}

pub fn verify_password(author: &Author, password: &str) -> bool {
    match PasswordHash::new(&author.encrypted_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        _ => false,
    }
}
