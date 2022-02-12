use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;
// use background_jobs::QueueHandle;

use crate::multipart::{get_file, parse_multipart};
use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::{actions, form_from_params};
use crate::models::{generate_pictures, NewPicture};

use crate::uri_helpers::*;

#[derive(Template)]
#[template(path = "pictures/new.html.jinja")]
struct New<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: &'a NewPicture,
    error: &'a Option<String>,
}

#[get("/pictures/new")]
pub async fn new(ident: Identity) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let s = New {
        title: Some("New picture"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: &None,
        form_data: &NewPicture {
            posse: true,
            show_in_index: true,
            lang: "en".to_owned(),
            ..Default::default()
        },
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/pictures")]
pub async fn create(
    ident: Identity,
    pool: web::Data<DbPool>,
    // queue: web::Data<QueueHandle>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let params = parse_multipart(&mut payload).await?;

    let (filename, file) = match get_file(&params) {
        Some((filename, file)) => (filename, file),
        _ => return Err(error::ErrorBadRequest("picture field is not a file")),
    };

    let content_type = match new_mime_guess::from_path(&filename).first_raw() {
        Some(s) => s,
        None => "image/jpeg",
    };
    let len = file.metadata()?.len();

    let form = form_from_params(
        &params,
        ident.identity().unwrap().parse::<i32>().unwrap(),
        &Some((filename, content_type.to_owned(), len as i32)),
    );

    let data = form.clone();
    let mut f = file.try_clone()?;
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_picture(&data, &mut f, &conn)
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

        let s = New {
            title: Some("New picture"),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}
