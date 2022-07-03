use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::{Deafie, NewDeafie};

use crate::uri_helpers::*;
// use crate::utils as filters;

#[derive(Template)]
#[template(path = "deafies/edit.html.jinja")]
struct Edit<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    deafie: &'a Deafie,
    form_data: &'a NewDeafie,
    error: &'a Option<String>,
}

#[get("/deaf-dog-training/{id}/edit")]
pub async fn edit(ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let deafie = web::block(move || {
        let conn = pool.get()?;
        actions::get_deafie(id.into_inner(), false, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Edit {
        lang: "de",
        title: Some(&format!("Edit deafie „{}“", deafie.title)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        deafie: &deafie,

        form_data: &NewDeafie {
            title: deafie.title.clone(),
            slug: deafie.slug.clone(),
            excerpt: deafie.excerpt.clone(),
            body: deafie.body.clone(),
            published: deafie.published,
            ..Default::default()
        },
        error: &None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/deaf-dog-training/{id}")]
pub async fn update(
    ident: Identity,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    form: web::Form<NewDeafie>,
) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let pool_ = pool.clone();
    let deafie = web::block(move || {
        let conn = pool_.get()?;
        actions::get_deafie(id.into_inner(), false, &conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.identity().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::update_deafie(deafie.id, &data, &conn)
    })
    .await?;

    if let Ok(deafie) = res {
        let uri = deafie_uri(&deafie);

        tokio::task::spawn_blocking(move || {
            let uri = deafie_uri(&deafie);
            let _ = send_mentions(&uri);
        });

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = Edit {
            lang: "de",
            title: Some(&format!("Edit deafie „{}“", deafie.title)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            deafie: &deafie,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}
