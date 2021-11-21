use actix_web::{error, get, web, Error, HttpResponse, Result};

use crate::DbPool;

use crate::notes::actions;

#[get("/notes/{id}")]
pub async fn show(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let note = web::block(move || {
        let conn = pool.get()?;
        actions::get_note(id.into_inner(), true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut ctx = tera::Context::new();
    ctx.insert("note", &note);
    ctx.insert("title", &format!("Note #{}: {}", note.id, note.title));
    ctx.insert("index", &false);
    ctx.insert("atom", &false);
    ctx.insert("page_type", "blog");

    let s = tmpl
        .render("notes/show.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
