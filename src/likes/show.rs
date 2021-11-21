use actix_web::{error, get, web, Error, HttpResponse, Result};

use crate::DbPool;

use super::actions;

#[get("/likes/{id}")]
pub async fn show(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let like = web::block(move || {
        let conn = pool.get()?;
        actions::get_like(id.into_inner(), true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut ctx = tera::Context::new();
    ctx.insert("like", &like);
    ctx.insert("title", &format!("♥ {}", like.in_reply_to));
    ctx.insert("index", &false);
    ctx.insert("atom", &false);

    let s = tmpl
        .render("likes/show.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
