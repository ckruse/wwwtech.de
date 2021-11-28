use argonautica::Verifier;
use diesel::prelude::*;

use crate::models::Author;
use crate::DbError;

pub fn get_author_by_email(user_email: &str, conn: &PgConnection) -> Result<Author, DbError> {
    use crate::schema::authors::dsl::*;

    let author = authors.filter(email.eq(user_email)).first::<Author>(conn)?;

    Ok(author)
}

pub fn verify_password(author: &Author, password: &str) -> bool {
    let mut verifier = Verifier::default();
    verifier
        .with_hash(author.encrypted_password.clone())
        .with_password(password)
        // .with_secret_key(secret_key)
        .verify()
        .unwrap()
}
