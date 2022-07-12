use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::multipart::{get_file, parse_multipart};
use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use super::form_from_params;
use crate::models::{generate_deafie_pictures, NewDeafie};

use crate::uri_helpers::*;

#[derive(Template)]
#[template(path = "deafies/new.html.jinja")]
struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: &'a NewDeafie,
    error: &'a Option<String>,
}

#[get("/the-life-of-alfons/new")]
pub(crate) async fn new(_ident: Identity) -> Result<HttpResponse, Error> {
    let s = New {
        lang: "de",
        title: Some("New article"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: &None,
        form_data: &NewDeafie { ..Default::default() },
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/the-life-of-alfons")]
pub(crate) async fn create(
    ident: Identity,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let params = parse_multipart(&mut payload).await?;

    let (filename, file, content_type) = match get_file(&params) {
        Some((filename, file)) => {
            let content_type = match new_mime_guess::from_path(&filename).first_raw() {
                Some(s) => s,
                None => "image/jpeg",
            };

            (Some(filename), Some(file), Some(content_type))
        }
        _ => (None, None, None),
    };

    let form = form_from_params(
        &params,
        ident.id().unwrap().parse::<i32>().unwrap(),
        filename,
        content_type,
    );

    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());
    let f = if let Some(f) = file { Some(f.try_clone()?) } else { None };

    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_deafie(&data, f, &conn)
    })
    .await?;

    if let Ok(deafie) = res {
        let uri = deafie_uri(&deafie);

        tokio::task::spawn_blocking(move || {
            let _ = generate_deafie_pictures(&deafie);

            if deafie.published {
                let uri = deafie_uri(&deafie);
                let _ = send_mentions(&uri);
            }
        });

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = New {
            lang: "de",
            title: Some("New article"),
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
