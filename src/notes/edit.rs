use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::{NewNote, Note};

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "notes/edit.html.jinja")]
struct Edit<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    note: &'a Note,
    form_data: &'a NewNote,
    error: &'a Option<String>,
}

#[get("/notes/{id}/edit")]
pub async fn edit(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let note = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_note(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Edit {
        lang: "en",
        title: Some(&format!("Edit note #{}: {}", note.id, note.title)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        note: &note,

        form_data: &NewNote {
            author_id: None,
            title: note.title.clone(),
            note_type: note.note_type.clone(),
            in_reply_to: note.in_reply_to.clone(),
            lang: note.lang.clone(),
            posse: note.posse,
            show_in_index: note.show_in_index,
            content: Some(note.content.clone()),
            inserted_at: None,
            updated_at: None,
            posse_visibility: note.posse_visibility.clone(),
            content_warning: note.content_warning.clone(),
        },
        error: &None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/notes/{id}")]
pub async fn update(
    ident: Identity,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    form: web::Form<NewNote>,
) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let note = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_note(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_note(note.id, &data, &mut conn)
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

        let s = Edit {
            lang: "en",
            title: Some("Edit note"),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            note: &note,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}
