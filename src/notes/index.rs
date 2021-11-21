use actix_web::{error, get, web, Error, HttpResponse, Result};

use crate::models::Note;
use crate::utils::paging::{get_page, get_paging, PageParams};
use crate::DbPool;

use crate::notes::{actions, PER_PAGE};

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

    let grouped_notes: Vec<Vec<Note>> = {
        let mut groups = Vec::new();
        let mut this_group: Vec<Note> = Vec::new();

        for note in notes {
            if this_group.is_empty() || this_group[0].inserted_at.date() == note.inserted_at.date() {
                this_group.push(note);
            } else {
                groups.push(this_group);
                this_group = vec![note];
            }
        }
        groups
    };

    let paging = get_paging(count, p, PER_PAGE);

    let mut ctx = tera::Context::new();
    ctx.insert("notes", &grouped_notes);
    ctx.insert("count", &count);
    ctx.insert("paging", &paging);
    ctx.insert("title", "Notes");
    ctx.insert("index", &true);
    ctx.insert("atom", &false);

    let s = tmpl
        .render("notes/index.html.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
