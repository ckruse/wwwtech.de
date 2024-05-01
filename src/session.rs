use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, uri_helpers::*, AppRouter, AppState, AuthSession};

pub mod actions;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Template)]
#[template(path = "login.html.jinja")]
pub struct Show<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    email: String,
}

pub fn configure(app: AppRouter) -> AppRouter {
    app.route("/login", get(new_session))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

pub async fn new_session() -> Show<'static> {
    Show {
        lang: "en",
        title: Some("Login"),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in: false,
        email: "".to_owned(),
    }
}

pub async fn login(
    mut auth: AuthSession,
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Response, AppError> {
    let mut conn = state.pool.acquire().await?;
    let author = actions::get_author_by_email(&form.email, &mut conn).await?;

    if actions::verify_password(&author, &form.password) {
        auth.login(&author)
            .await
            .map_err(|e| AppError::InternalError(format!("error logging in: {}", e)))?;

        Ok(Redirect::to(&root_uri()).into_response())
    } else {
        Ok((
            StatusCode::UNAUTHORIZED,
            Show {
                lang: "en",
                title: Some("Login"),
                page_type: None,
                page_image: None,
                body_id: None,
                logged_in: false,
                email: form.email,
            },
        )
            .into_response())
    }
}

pub async fn logout(mut auth: AuthSession) -> Result<impl IntoResponse, AppError> {
    let _ = auth.logout().await;
    Ok(Redirect::to(&root_uri()))
}
