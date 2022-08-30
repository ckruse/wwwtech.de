use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

// use crate::webmentions::send::WebmenentionSenderJob;
use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::{Like, NewLike};

use crate::uri_helpers::*;
// use crate::utils as filters;

#[derive(Template)]
#[template(path = "likes/edit.html.jinja")]
struct Edit<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    like: &'a Like,
    form_data: &'a NewLike,
    error: &'a Option<String>,
}

#[get("/likes/{id}/edit")]
pub async fn edit(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let like = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_like(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Edit {
        lang: "en",
        title: Some(&format!("Edit like #{}", like.id)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        like: &like,

        form_data: &NewLike {
            author_id: None,
            in_reply_to: like.in_reply_to.clone(),
            posse: like.posse,
            show_in_index: like.show_in_index,
            inserted_at: None,
            updated_at: None,
        },
        error: &None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/likes/{id}")]
pub async fn update(
    ident: Identity,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    form: web::Form<NewLike>,
) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let like = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_like(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_like(like.id, &data, &mut conn)
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

        let s = Edit {
            lang: "en",
            title: Some(&format!("Edit like #{}", like.id)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            like: &like,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}
