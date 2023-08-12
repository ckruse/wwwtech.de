use askama::Template;
use axum::{
    extract::{Extension, Form, State},
    response::{IntoResponse, Redirect, Response},
};

use super::actions;
use crate::{
    errors::AppError, models::Author, models::NewArticle, posse::mastodon::post_article, uri_helpers::*,
    utils as filters, webmentions::send::send_mentions, AppState,
};

#[derive(Template)]
#[template(path = "articles/new.html.jinja")]
pub struct New<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,
    form_data: NewArticle,
    error: Option<String>,
}

pub(crate) async fn new() -> New<'static> {
    New {
        lang: "en",
        title: Some("New article"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,
        error: None,
        form_data: NewArticle {
            lang: "en".to_owned(),
            posse: true,
            ..Default::default()
        },
    }
}

pub(crate) async fn create(
    Extension(user): Extension<Author>,
    State(state): State<AppState>,
    Form(mut form): Form<NewArticle>,
) -> Result<Response, AppError> {
    let mut conn = state.pool.acquire().await?;
    form.author_id = Some(user.id);
    let res = actions::create_article(&form, &mut conn).await;

    if let Ok(article) = res {
        let uri = article_uri(&article);

        if article.published {
            if article.posse {
                let article_ = article.clone();
                tokio::task::spawn(async move {
                    let _ = post_article(&article_).await;
                });
            }

            tokio::task::spawn_blocking(move || {
                let uri = article_uri(&article);
                let _ = send_mentions(&uri);
            });
        }

        Ok(Redirect::to(&uri).into_response())
    } else {
        let error = match res {
            Err(cause) => Some(cause.to_string()),
            Ok(_) => None,
        };

        Ok(New {
            lang: "en",
            title: Some("New article"),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in: true,
            form_data: form,
            error,
        }
        .into_response())
    }
}
