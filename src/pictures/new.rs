use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;
use std::collections::HashMap;
use std::fs::File;

use crate::multipart::{parse_multipart, MultipartField};
use crate::DbPool;

use super::actions;
use crate::models::NewPicture;

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

fn get_file(params: &HashMap<String, MultipartField>) -> Result<(String, &File), Error> {
    let field = params
        .get("picture")
        .ok_or_else(|| HttpResponse::BadRequest().finish())?;

    if let MultipartField::File(filename, file) = field {
        Ok((filename.clone(), file))
    } else {
        Err(error::ErrorBadRequest("picture field is not a file"))
    }
}

#[post("/pictures")]
pub async fn create(ident: Identity, pool: web::Data<DbPool>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let params = parse_multipart(&mut payload).await?;
    // println!("params: {:?}", params);

    let (filename, file) = get_file(&params)?;
    let content_type = match new_mime_guess::from_path(&filename).first_raw() {
        Some(s) => s,
        None => "image/jpeg",
    };
    let len = file.metadata()?.len();

    let form = NewPicture {
        author_id: Some(ident.identity().unwrap().parse::<i32>().unwrap()),
        title: match params.get("title") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        alt: match params.get("alt") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        lang: match params.get("lang") {
            Some(MultipartField::Form(v)) => v.clone(),
            _ => "".to_owned(),
        },
        in_reply_to: match params.get("in_reply_to") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        posse: match params.get("posse") {
            Some(MultipartField::Form(v)) => v == "true",
            _ => false,
        },
        show_in_index: match params.get("show_in_index") {
            Some(MultipartField::Form(v)) => v == "true",
            _ => false,
        },
        content: match params.get("content") {
            Some(MultipartField::Form(v)) => Some(v.clone()),
            _ => None,
        },
        image_file_name: Some(filename.clone()),
        image_content_type: Some(content_type.to_owned()),
        image_file_size: Some(len as i32),
        ..Default::default()
    };

    let data = form.clone();
    let mut f = file.try_clone()?;
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_picture(&data, &mut f, &conn)
    })
    .await;

    if let Ok(picture) = res {
        Ok(HttpResponse::Found()
            .header(header::LOCATION, picture_uri(&picture))
            .finish())
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
