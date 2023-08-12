use askama::Template;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use sqlx::PgConnection;

use super::actions;
use crate::{errors::AppError, models::Article, uri_helpers::*, utils as filters, AppState, AuthContext};

#[derive(Template)]
#[template(path = "articles/show.html.jinja")]
pub struct Show<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    article: Article,
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

pub async fn show(
    auth: AuthContext,
    State(state): State<AppState>,
    Path((year, month, slug)): Path<(i32, String, String)>,
) -> Result<Response, AppError> {
    let logged_in = auth.current_user.is_some();
    let guid = format!("{}/{}/{}", year, month, slug);
    let mut conn = state.pool.acquire().await?;

    let Some(article) = actions::get_article_by_slug(&guid, !logged_in, &mut conn).await? else {
        return redirect_or_error(slug, &mut conn, logged_in).await;
    };

    Ok(Show {
        lang: "en",
        title: Some(article.title.clone()),
        page_type: Some("blog"),
        page_image: None,
        body_id: None,
        logged_in,
        article,
        index: false,
        atom: false,
    }
    .into_response())
}
