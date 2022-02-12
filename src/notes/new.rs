use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::NewNote;

use crate::uri_helpers::*;

#[derive(Template)]
#[template(path = "notes/new.html.jinja")]
struct New<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: &'a NewNote,
    error: &'a Option<String>,
}

#[get("/notes/new")]
pub async fn new(ident: Identity) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let s = New {
        title: Some("New note"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: &None,
        form_data: &NewNote {
            note_type: "note".to_owned(),
            lang: "en".to_owned(),
            posse: true,
            show_in_index: true,
            ..Default::default()
        },
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/notes")]
pub async fn create(ident: Identity, pool: web::Data<DbPool>, form: web::Form<NewNote>) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let mut data = form.clone();
    data.author_id = Some(ident.identity().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_note(&data, &conn)
    })
    .await?;

    if let Ok(note) = res {
        let uri = note_uri(&note);

        tokio::task::spawn_blocking(move || {
            let uri = note_uri(&note);
            let _ = send_mentions(&uri);
        });

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = New {
            title: Some("New note"),
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
