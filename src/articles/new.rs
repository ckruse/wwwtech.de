use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::DbPool;

use super::actions;
use crate::models::NewArticle;

use crate::uri_helpers::*;

#[derive(Template)]
#[template(path = "articles/new.html.jinja")]
struct New<'a> {
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: &'a NewArticle,
    error: &'a Option<String>,
}

#[get("/articles/new")]
pub(crate) async fn new(ident: Identity) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let s = New {
        title: Some("New article"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: &None,
        form_data: &NewArticle {
            lang: "en".to_owned(),
            posse: true,
            ..Default::default()
        },
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/articles")]
pub(crate) async fn create(
    ident: Identity,
    pool: web::Data<DbPool>,
    form: web::Form<NewArticle>,
) -> Result<HttpResponse, Error> {
    if ident.identity().is_none() {
        return Result::Err(error::ErrorForbidden("You have to be logged in to see this page"));
    }

    let mut data = form.clone();
    data.author_id = Some(ident.identity().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_article(&data, &conn)
    })
    .await;

    if let Ok(article) = res {
        Ok(HttpResponse::Found()
            .header(header::LOCATION, article_uri(&article))
            .finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = New {
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
