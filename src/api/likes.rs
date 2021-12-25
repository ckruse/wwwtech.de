use actix_web::{delete, error, get, post, put, web, Responder, Result};
use background_jobs::QueueHandle;
use chrono::Timelike;

use crate::models::{Like, NewLike};
use crate::uri_helpers::like_uri;
use crate::utils::paging::{get_page, PageParams};
use crate::DbPool;

use crate::likes::actions;
use crate::webmentions::send::WebmenentionSenderJob;

static PER_PAGE: i64 = 50;

#[get("/likes.json")]
pub async fn index(pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<impl Responder> {
    let p = get_page(&page);

    let likes = web::block(move || {
        let conn = pool.get()?;
        actions::list_likes(PER_PAGE, p * PER_PAGE, false, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?
    .iter()
    .map(|like| {
        let mut like = like.clone();
        like.inserted_at = like.inserted_at.with_nanosecond(0).unwrap();
        like.updated_at = like.updated_at.with_nanosecond(0).unwrap();
        like
    })
    .collect::<Vec<Like>>();

    Ok(web::Json(likes))
}

#[post("/likes.json")]
pub async fn create(
    pool: web::Data<DbPool>,
    queue: web::Data<QueueHandle>,
    form: web::Json<NewLike>,
) -> Result<impl Responder> {
    let mut data = form.clone();
    data.author_id = Some(1);
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_like(&data, &conn)
    })
    .await;

    if let Ok(like) = res {
        let uri = like_uri(&like);
        let _ = queue.queue(WebmenentionSenderJob {
            source_url: uri.clone(),
        });
        Ok(web::Json(like))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[put("/likes/{id}.json")]
pub async fn update(
    pool: web::Data<DbPool>,
    queue: web::Data<QueueHandle>,
    id: web::Path<i32>,
    form: web::Json<NewLike>,
) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let like = web::block(move || {
        let conn = pool_.get()?;
        actions::get_like(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(1);
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::update_like(like.id, &data, &conn)
    })
    .await;

    if let Ok(like) = res {
        let uri = like_uri(&like);
        let _ = queue.queue(WebmenentionSenderJob {
            source_url: uri.clone(),
        });
        Ok(web::Json(like))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[delete("/likes/{id}.json")]
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let like = web::block(move || {
        let conn = pool_.get()?;
        actions::get_like(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let like_id = like.id;
    let _deleted = web::block(move || {
        let conn = pool.get()?;
        actions::delete_like(like_id, &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(web::Json(like))
}