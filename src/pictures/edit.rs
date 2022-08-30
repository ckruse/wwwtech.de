use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::multipart::{get_file, parse_multipart};
use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::{actions, form_from_params};
use crate::models::{generate_pictures, NewPicture, Picture};

use crate::uri_helpers::*;
// use crate::utils as filters;

#[derive(Template)]
#[template(path = "pictures/edit.html.jinja")]
struct Edit<'a> {
    lang: &'a str,
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
pub async fn edit(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let picture = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_picture(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Edit {
        lang: "en",
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
    id: web::Path<i32>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
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

    let form = form_from_params(&params, ident.id().unwrap().parse::<i32>().unwrap(), &metadata);

    let pool_ = pool.clone();
    let picture = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_picture(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());

    let picture_ = picture.clone();
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_picture(&picture_, &data, &metadata, &mut file, &mut conn)
    })
    .await?;

    if let Ok(picture) = res {
        let uri = picture_uri(&picture);

        tokio::task::spawn_blocking(move || {
            let uri = picture_uri(&picture);
            let _ = generate_pictures(&picture);
            let _ = send_mentions(&uri);
        });

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = Edit {
            lang: "en",
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
