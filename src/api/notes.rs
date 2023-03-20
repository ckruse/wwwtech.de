use actix_web::{delete, error, get, post, put, web, Responder, Result};
use chrono::Timelike;

use crate::models::{NewNote, Note};
use crate::posse::mastodon::post_note;
use crate::uri_helpers::note_uri;
use crate::utils::paging::{get_page, PageParams};
use crate::DbPool;

use crate::notes::actions;
use crate::webmentions::send::send_mentions;

static PER_PAGE: i64 = 50;

#[get("/notes.json")]
pub async fn index(pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<impl Responder> {
    let p = get_page(&page);

    let notes = web::block(move || {
        let mut conn = pool.get()?;
        actions::list_notes(PER_PAGE, p * PER_PAGE, false, &mut conn)
    })
    .await?
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
pub async fn create(pool: web::Data<DbPool>, form: web::Json<NewNote>) -> Result<impl Responder> {
    let mut data = form.clone();
    data.author_id = Some(1);
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::create_note(&data, &mut conn)
    })
    .await?;

    if let Ok(note) = res {
        let uri = note_uri(&note);

        if note.posse {
            let note_ = note.clone();
            tokio::task::spawn(async move {
                let _ = post_note(&note_).await;
            });
        }

        tokio::task::spawn_blocking(move || send_mentions(&uri));

        Ok(web::Json(note))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[put("/notes/{id}.json")]
pub async fn update(pool: web::Data<DbPool>, id: web::Path<i32>, form: web::Json<NewNote>) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let note = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_note(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(1);
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_note(note.id, &data, &mut conn)
    })
    .await?;

    if let Ok(note) = res {
        let uri = note_uri(&note);
        tokio::task::spawn_blocking(move || send_mentions(&uri));

        Ok(web::Json(note))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[delete("/notes/{id}")]
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let note = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_note(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let note_id = note.id;
    let _deleted = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_note(note_id, &mut conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(web::Json(note))
}
