use actix_identity::Identity;
use actix_web::{error, http::header, post, web, Error, HttpResponse, Result};

use crate::DbPool;

use super::actions;

use crate::uri_helpers::*;

#[post("/the-life-of-alfons/{id}/delete")]
pub async fn delete(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let deafie = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_deafie(id.into_inner(), false, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let _deleted = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_deafie(deafie.id, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, deafies_uri()))
        .finish())
}
