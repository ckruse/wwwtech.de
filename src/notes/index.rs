use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;

use crate::models::Note;
use crate::utils::paging::{get_page, get_paging, PageParams, Paging};
use crate::DbPool;

use super::{actions, PER_PAGE};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "notes/index.html.jinja")]
struct Index<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    notes: &'a Vec<Vec<Note>>,
    paging: &'a Paging,
    index: bool,
    atom: bool,
}

#[get("/notes")]
pub async fn index(id: Identity, pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<HttpResponse, Error> {
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

    let s = Index {
        title: Some("Notes"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: id.identity().is_some(),
        notes: &grouped_notes,
        paging: &paging,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
