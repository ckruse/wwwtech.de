use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;
use diesel::prelude::*;

use crate::models::Author;
use crate::DbError;

pub fn get_author_by_email(user_email: &str, conn: &PgConnection) -> Result<Author, DbError> {
    use crate::schema::authors::dsl::*;

    let author = authors.filter(email.eq(user_email)).first::<Author>(conn)?;

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
