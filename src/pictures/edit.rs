use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;
use background_jobs::QueueHandle;

use crate::multipart::{get_file, parse_multipart};
use crate::webmentions::send::WebmenentionSenderJob;
use crate::DbPool;

use super::{actions, form_from_params};
use crate::models::{NewPicture, Picture};

use crate::uri_helpers::*;
// use crate::utils as filters;

#[derive(Template)]
#[template(path = "pictures/edit.html.jinja")]
struct Edit<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    picture: &'a Picture,
    form_data: &'a NewPicture,
    error: &'a Option<String>,
}

#[get("/pictures/{id}/edit")]
pub async fn edit(ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let picture = web::block(move || {
        let conn = pool.get()?;
        actions::get_picture(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Edit {
        title: Some(&format!("Edit picture #{}", picture.id)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        picture: &picture,

        form_data: &NewPicture {
            author_id: None,
            title: picture.title.clone(),
            alt: picture.alt.clone(),
            in_reply_to: picture.in_reply_to.clone(),
            lang: picture.lang.clone(),
            posse: picture.posse,
            show_in_index: picture.show_in_index,
            content: Some(picture.content.clone()),
            ..Default::default()
        },
        error: &None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/pictures/{id}")]
pub async fn update(
    ident: Identity,
    pool: web::Data<DbPool>,
    queue: web::Data<QueueHandle>,
    id: web::Path<i32>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let params = parse_multipart(&mut payload).await?;
    let (metadata, mut file) = match get_file(&params) {
        Some((filename, file)) => {
            let content_type = match new_mime_guess::from_path(&filename).first_raw() {
                Some(s) => s,
                None => "image/jpeg",
            };
            let len = file.metadata()?.len();
            (
                Some((filename.clone(), content_type.to_owned(), len as i32)),
                Some(file.try_clone()?),
            )
        }
        _ => (None, None),
    };

    let form = form_from_params(&params, ident.identity().unwrap().parse::<i32>().unwrap(), &metadata);

    let pool_ = pool.clone();
    let picture = web::block(move || {
        let conn = pool_.get()?;
        actions::get_picture(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.identity().unwrap().parse::<i32>().unwrap());

    let picture_ = picture.clone();
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::update_picture(&picture_, &data, &metadata, &mut file, &conn)
    })
    .await;

    if let Ok(picture) = res {
        let uri = picture_uri(&picture);
        let _ = queue.queue(picture);
        let _ = queue.queue(WebmenentionSenderJob {
            source_url: uri.clone(),
        });
        Ok(HttpResponse::Found().header(header::LOCATION, uri).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = Edit {
            title: Some(&format!("Edit picture #{}", picture.id)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            picture: &picture,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}