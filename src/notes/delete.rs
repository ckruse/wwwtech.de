use actix_identity::Identity;
use actix_web::{error, http::header, post, web, Error, HttpResponse, Result};

use crate::DbPool;

use super::actions;

use crate::uri_helpers::*;

#[post("/notes/{id}/delete")]
pub async fn delete(ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let pool_ = pool.clone();
    let note = web::block(move || {
        let conn = pool_.get()?;
        actions::get_note(id.into_inner(), true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let _deleted = web::block(move || {
        let conn = pool.get()?;
        actions::delete_note(note.id, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(HttpResponse::Found().header(header::LOCATION, notes_uri()).finish())
}
