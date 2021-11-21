use actix_web::{error, get, web, Error, HttpResponse, Result};

use crate::utils::paging::{get_page, get_paging, PageParams};
use crate::DbPool;

use crate::pictures::{actions, PER_PAGE};

#[get("/pictures")]
pub async fn index(
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
    page: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let p = get_page(&page);

    let pool_ = pool.clone();
    let pictures = web::block(move || {
        let conn = pool_.get()?;
        actions::list_pictures(PER_PAGE, p * PER_PAGE, true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let conn = pool.get()?;
        actions::count_pictures(true, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let mut ctx = tera::Context::new();
    ctx.insert("pictures", &pictures);
    ctx.insert("count", &count);
    ctx.insert("paging", &paging);
    ctx.insert("title", "Pictures");
    ctx.insert("index", &true);
    ctx.insert("type", "thumbnail");
    ctx.insert("body_id", "pictures-list");

    let s = tmpl
        .render("pictures/index.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
