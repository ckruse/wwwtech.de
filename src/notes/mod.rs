use actix_web::{error, get, web, Error, HttpResponse, Result};

use crate::utils::{get_page, PageParams};
use crate::DbPool;

pub mod actions;

static PER_PAGE: i64 = 25;

#[get("/notes")]
pub async fn index(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    page: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let pool_ = pool.clone();
    let notes = web::block(move || {
        let conn = pool_.get()?;
        actions::list_notes(PER_PAGE, p * PER_PAGE, true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_notes(true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut ctx = tera::Context::new();
    ctx.insert("notes", &notes);
    ctx.insert("count", &count);

    let s = tmpl
        .render("pages/index.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s))
}
