use actix_web::{delete, error, get, post, put, web, Responder, Result};
// use background_jobs::QueueHandle;
use chrono::Timelike;
use std::fs::File;
use std::io::Write;
use tempfile::tempfile;

use crate::models::{generate_pictures, NewJsonPicture, Picture};
use crate::uri_helpers::picture_uri;
use crate::utils::paging::{get_page, PageParams};
use crate::DbPool;

use crate::pictures::actions;
use crate::webmentions::send::send_mentions;

const PER_PAGE: i64 = 50;

#[get("/pictures.json")]
pub async fn index(pool: web::Data<DbPool>, page: web::Query<PageParams>) -> Result<impl Responder> {
    let p = get_page(&page);

    let pictures = web::block(move || {
        let mut conn = pool.get()?;
        actions::list_pictures(PER_PAGE, p * PER_PAGE, true, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?
    .iter()
    .map(|pic| {
        let mut pic = pic.clone();
        pic.inserted_at = pic.inserted_at.with_nanosecond(0).unwrap();
        pic.updated_at = pic.updated_at.with_nanosecond(0).unwrap();
        pic
    })
    .collect::<Vec<Picture>>();

    Ok(web::Json(pictures))
}

#[post("/pictures.json")]
pub async fn create(pool: web::Data<DbPool>, form: web::Json<NewJsonPicture>) -> Result<impl Responder> {
    let mut data = form.new_picture.clone();

    let file = web::block(move || -> Result<File, anyhow::Error> {
        let mut file = tempfile().map_err(|_| anyhow!("could not create tempfile"))?;
        let file_data = base64::decode(form.picture.as_ref().unwrap()).map_err(|_| anyhow!("could not decode file"))?;

        file.write_all(&file_data)
            .map_err(|_| anyhow!("could not write file"))?;

        Ok(file)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("file error: {}", e)))?;

    let len = file.metadata()?.len();
    data.author_id = Some(1);
    data.image_file_name = Some("image.jpg".to_owned());
    data.image_content_type = Some("image/jpeg".to_owned());
    data.image_file_size = Some(len as i32);

    let mut f = file.try_clone()?;
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::create_picture(&data, &mut f, &mut conn)
    })
    .await?;

    if let Ok(picture) = res {
        let pic = picture.clone();

        tokio::task::spawn_blocking(move || {
            let uri = picture_uri(&pic);
            let _ = generate_pictures(&pic);
            let _ = send_mentions(&uri);
        });

        Ok(web::Json(picture))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[put("/pictures/{id}.json")]
pub async fn update(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    form: web::Json<NewJsonPicture>,
) -> Result<impl Responder> {
    let mut data = form.new_picture.clone();
    let pool_ = pool.clone();
    let picture = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_picture(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut file = web::block(move || -> Result<Option<File>, anyhow::Error> {
        if let Some(input) = form.picture.as_ref() {
            let mut file = tempfile().map_err(|_| anyhow!("could not create tempfile"))?;
            let file_data = base64::decode(input).map_err(|_| anyhow!("could not decode file"))?;
            file.write_all(&file_data)
                .map_err(|_| anyhow!("could not write file"))?;

            Ok(Some(file))
        } else {
            Ok(None)
        }
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("file error: {}", e)))?;

    data.author_id = Some(1);
    let metadata = match file.as_ref() {
        Some(file) => {
            let len = file.metadata().unwrap().len();
            Some(("image.jpg".to_owned(), "image/jpeg".to_owned(), len as i32))
        }
        _ => None,
    };

    let picture_ = picture.clone();
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_picture(&picture_, &data, &metadata, &mut file, &mut conn)
    })
    .await?;

    if let Ok(picture) = res {
        let pic = picture.clone();
        tokio::task::spawn_blocking(move || {
            let uri = picture_uri(&pic);
            let _ = generate_pictures(&pic);
            let _ = send_mentions(&uri);
        });

        Ok(web::Json(picture))
    } else {
        Err(error::ErrorInternalServerError("something went wrong"))
    }
}

#[delete("/pictures/{id}.json")]
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<impl Responder> {
    let pool_ = pool.clone();
    let picture = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_picture(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let pic = picture.clone();
    let _deleted = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_picture(&pic, &mut conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    Ok(web::Json(picture))
}
