use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::multipart::{get_file, parse_multipart};
use crate::posse::mastodon::post_deafie;
use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::{actions, form_from_params};
use crate::models::{generate_deafie_pictures, Deafie, NewDeafie};

use crate::uri_helpers::*;
use crate::utils as filters;

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

#[get("/the-life-of-alfons/{id}/edit")]
pub async fn edit(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let deafie = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_deafie(id.into_inner(), false, &mut conn)
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
            posse_visibility: deafie.posse_visibility.clone(),
            content_warning: deafie.content_warning.clone(),
            ..Default::default()
        },
        error: &None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/the-life-of-alfons/{id}")]
pub async fn update(
    ident: Identity,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let params = parse_multipart(&mut payload).await?;
    let (filename, file, content_type) = match get_file(&params) {
        Some((filename, file)) => {
            let content_type = new_mime_guess::from_path(&filename).first_raw().unwrap_or("image/jpeg");
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

    let pool_ = pool.clone();
    let old_deafie = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_deafie(id.into_inner(), false, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());
    let f = if let Some(f) = file { Some(f.try_clone()?) } else { None };
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_deafie(old_deafie.id, &data, f, &mut conn)
    })
    .await?;

    if let Ok(deafie) = res {
        let uri = deafie_uri(&deafie);

        tokio::task::spawn_blocking(move || {
            let _ = generate_deafie_pictures(&deafie);

            if deafie.published {
                let uri = deafie_uri(&deafie);
                let _ = send_mentions(&uri);

                if !old_deafie.published {
                    tokio::task::spawn(async move {
                        let _ = post_deafie(&deafie).await;
                    });
                }
            }
        });

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = Edit {
            lang: "de",
            title: Some(&format!("Edit deafie „{}“", old_deafie.title)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            deafie: &old_deafie,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}
