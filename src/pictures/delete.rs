use actix_identity::Identity;
use actix_web::{error, http::header, post, web, Error, HttpResponse, Result};

use crate::DbPool;

use super::actions;

use crate::uri_helpers::*;

#[post("/pictures/{id}/delete")]
pub async fn delete(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let picture = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_picture(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let _deleted = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_picture(&picture, &mut conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, pictures_uri()))
        .finish())
}
