use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::NewLike;

use crate::uri_helpers::*;

#[derive(Template)]
#[template(path = "likes/new.html.jinja")]
struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: &'a NewLike,
    error: &'a Option<String>,
}

#[get("/likes/new")]
pub async fn new(ident: Identity) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let s = New {
        lang: "en",
        title: Some("New like"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: &None,
        form_data: &NewLike {
            posse: true,
            show_in_index: true,
            ..Default::default()
        },
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/likes")]
pub async fn create(ident: Identity, pool: web::Data<DbPool>, form: web::Form<NewLike>) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let mut data = form.clone();
    data.author_id = Some(ident.identity().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_like(&data, &conn)
    })
    .await?;

    if let Ok(like) = res {
        let uri = like_uri(&like);

        tokio::task::spawn_blocking(move || {
            let uri = like_uri(&like);
            let _ = send_mentions(&uri);
        });

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = New {
            lang: "en",
            title: Some("New like"),
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
