use actix_web::{delete, error, get, post, put, web, Responder, Result};
use background_jobs::QueueHandle;
use chrono::Timelike;

use crate::models::{NewNote, Note};
use crate::uri_helpers::note_uri;
use crate::utils::paging::{get_page, PageParams};
use crate::DbPool;

use crate::notes::actions;
use crate::webmentions::send::WebmenentionSenderJob;

static PER_PAGE: i64 = 50;

#[get("/notes.json")]
pub async fn index(pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<impl Responder> {
    let p = get_page(&page);

    let notes = web::block(move || {
        let conn = pool.get()?;
        actions::list_notes(PER_PAGE, p * PER_PAGE, false, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?
    .iter()
    .map(|note| {
        let mut note = note.clone();
        note.inserted_at = note.inserted_at.with_nanosecond(0).unwrap();
        note.updated_at = note.updated_at.with_nanosecond(0).unwrap();
        note
    })
    .collect::<Vec<Note>>();

    Ok(web::Json(notes))
}

#[post("/notes.json")]
pub async fn create(
    pool: web::Data<DbPool>,
    queue: web::Data<QueueHandle>,
    form: web::Json<NewNote>,
) -> Result<impl Responder> {
    let mut data = form.clone();
    data.author_id = Some(1);
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_note(&data, &conn)
    })
    .await;

    if let Ok(note) = res {
        let uri = note_uri(&note);
        let _ = queue.queue(WebmenentionSenderJob {
            source_url: uri.clone(),
        });

        Ok(web::Json(note))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[put("/notes/{id}.json")]
pub async fn update(
    pool: web::Data<DbPool>,
    queue: web::Data<QueueHandle>,
    id: web::Path<i32>,
    form: web::Json<NewNote>,
) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let note = web::block(move || {
        let conn = pool_.get()?;
        actions::get_note(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(1);
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::update_note(note.id, &data, &conn)
    })
    .await;

    if let Ok(note) = res {
        let uri = note_uri(&note);
        let _ = queue.queue(WebmenentionSenderJob {
            source_url: uri.clone(),
        });
        Ok(web::Json(note))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[delete("/notes/{id}")]
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let note = web::block(move || {
        let conn = pool_.get()?;
        actions::get_note(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let note_id = note.id;
    let _deleted = web::block(move || {
        let conn = pool.get()?;
        actions::delete_note(note_id, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(web::Json(note))
}
