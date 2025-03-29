use askama::Template;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};

use super::actions;
use crate::errors::AppError;
use crate::models::{Article, NewArticle};
use crate::posse::mastodon::post_article;
use crate::uri_helpers::*;
use crate::webmentions::send::send_mentions;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "articles/edit.html.j2")]
pub struct Edit<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    article: Article,
    form_data: NewArticle,
    error: Option<String>,
}

pub async fn edit(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let article = actions::get_article(id, false, &mut conn).await?;

    let html = Edit {
        lang: "en",
        title: Some(format!("Edit article „{}“", article.title)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: true,

        form_data: NewArticle {
            in_reply_to: article.in_reply_to.clone(),
            title: article.title.clone(),
            slug: article.slug.clone(),
            excerpt: article.excerpt.clone(),
            body: article.body.clone(),
            published: article.published,
            posse: article.posse,
            lang: article.lang.clone(),
            posse_visibility: article.posse_visibility.clone(),
            content_warning: article.content_warning.clone(),
            ..Default::default()
        },
        article,
        error: None,
    }
    .render()?;

    Ok(Html(html))
}

pub async fn update(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(mut form): Form<NewArticle>,
) -> Result<Response, AppError> {
    let Some(user) = auth.user else {
        return Err(AppError::Unauthorized);
    };

    let mut conn = state.pool.acquire().await?;
    let article = actions::get_article(id, false, &mut conn).await?;

    form.author_id = Some(user.id);

    match actions::update_article(article.id, &form, &mut conn).await {
        Ok(updated_article) => {
            state
                .article_cache
                .insert(updated_article.slug.clone(), updated_article.clone())
                .await;

            let uri = article_uri(&updated_article);

            if updated_article.published {
                if updated_article.posse && (!article.posse || !article.published) {
                    let article = updated_article.clone();
                    tokio::task::spawn(async move {
                        let _ = post_article(&article).await;
                    });
                }

                tokio::task::spawn_blocking(move || {
                    let uri = article_uri(&updated_article);
                    let _ = send_mentions(&uri);
                });
            }

            Ok(Redirect::to(&uri).into_response())
        }

        Err(cause) => {
            let html = Edit {
                lang: "en",
                title: Some(format!("Edit article „{}“", article.title)),
                page_type: None,
                page_image: None,
                body_id: None,
                logged_in: true,
                article,
                form_data: form,
                error: Some(cause.to_string()),
            }
            .render()?;

            Ok((StatusCode::UNPROCESSABLE_ENTITY, Html(html)).into_response())
        }
    }
}
