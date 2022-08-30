use actix_identity::Identity;
use actix_web::{error, get, http::header, post, web, Error, HttpResponse, Result};
use askama::Template;

// use crate::webmentions::send::WebmenentionSenderJob;
use crate::webmentions::send::send_mentions;
use crate::DbPool;

use super::actions;
use crate::models::{Article, NewArticle};

use crate::uri_helpers::*;
// use crate::utils as filters;

#[derive(Template)]
#[template(path = "articles/edit.html.jinja")]
struct Edit<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    article: &'a Article,
    form_data: &'a NewArticle,
    error: &'a Option<String>,
}

#[get("/articles/{id}/edit")]
pub async fn edit(_ident: Identity, pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let article = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_article(id.into_inner(), false, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let s = Edit {
        lang: "en",
        title: Some(&format!("Edit article „{}“", article.title)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        article: &article,

        form_data: &NewArticle {
            in_reply_to: article.in_reply_to.clone(),
            title: article.title.clone(),
            slug: article.slug.clone(),
            excerpt: article.excerpt.clone(),
            body: article.body.clone(),
            published: article.published,
            posse: article.posse,
            lang: article.lang.clone(),
            ..Default::default()
        },
        error: &None,
    }
    .render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[post("/articles/{id}")]
pub async fn update(
    ident: Identity,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    form: web::Form<NewArticle>,
) -> Result<HttpResponse, Error> {
    let pool_ = pool.clone();
    let article = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_article(id.into_inner(), false, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let mut data = form.clone();
    data.author_id = Some(ident.id().unwrap().parse::<i32>().unwrap());
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_article(article.id, &data, &mut conn)
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

        let s = Edit {
            lang: "en",
            title: Some(&format!("Edit article „{}“", article.title)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            article: &article,
            form_data: &form,
            error: &error,
        }
        .render()
        .unwrap();

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
    }
}
