use askama::Template;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use sqlx::PgConnection;

use super::actions;
use crate::errors::AppError;
use crate::models::Article;
use crate::uri_helpers::*;
use crate::{AppState, AuthSession, utils as filters};

#[derive(Template)]
#[template(path = "articles/show.html.j2")]
pub struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    article: &'a Article,
    index: bool,
    atom: bool,
}

async fn redirect_or_error(slug: String, conn: &mut PgConnection, logged_in: bool) -> Result<Response, AppError> {
    let article = actions::get_article_by_slug_part(&slug, !logged_in, conn).await?;

    match article {
        Some(article) => Ok(Redirect::permanent(&article_uri(&article)).into_response()),
        _ => Err(AppError::NotFound("article could not be found".to_owned())),
    }
}

async fn get_article(
    guid: &str,
    logged_in: bool,
    state: &AppState,
    db: &mut PgConnection,
) -> Result<Option<Article>, AppError> {
    let article = match state.article_cache.get(guid).await {
        Some(article) => Some(article),
        None => {
            let article = actions::get_article_by_slug(guid, !logged_in, db).await?;
            if let Some(ref article) = article {
                state.article_cache.insert(guid.to_owned(), article.clone()).await;
            }

            article
        }
    };

    Ok(article)
}

pub async fn show(
    auth: AuthSession,
    State(state): State<AppState>,
    Path((year, month, slug)): Path<(i32, String, String)>,
) -> Result<Response, AppError> {
    let logged_in = auth.user.is_some();
    let guid = format!("{}/{}/{}", year, month, slug);
    let mut conn = state.pool.acquire().await?;

    let Some(article) = get_article(&guid, logged_in, &state, &mut conn).await? else {
        return redirect_or_error(slug, &mut conn, logged_in).await;
    };

    Ok(Show {
        lang: "en",
        title: Some(&article.title),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in,
        article: &article,
        index: false,
        atom: false,
    }
    .into_response())
}
