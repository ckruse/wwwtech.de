use actix_identity::Identity;
use actix_web::{get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::NewArticle;

use crate::uri_helpers::*;

#[derive(Template)]
#[template(path = "articles/new.html.jinja")]
struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: &'a NewArticle,
    error: &'a Option<String>,
}

#[get("/articles/new")]
pub(crate) async fn new(_ident: Identity) -> Result<HttpResponse, Error> {
    let s = New {
        lang: "en",
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
    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let conn = pool.get()?;
        actions::create_article(&data, &conn)
    })
    .await?;

    if let Ok(article) = res {
        let uri = article_uri(&article);

        if article.published {
            tokio::task::spawn_blocking(move || {
                let uri = article_uri(&article);
                let _ = send_mentions(&uri);
            });
        }

        Ok(HttpResponse::Found().append_header((header::LOCATION, uri)).finish())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        let s = New {
            lang: "en",
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
