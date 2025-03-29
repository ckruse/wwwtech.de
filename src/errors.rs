use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AppError {
    DbError(sqlx::Error),
    NotFound(String),
    InternalError(String),
    BadRequest(String),
    Unauthorized,
    TemplateError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, body) = match self {
            AppError::DbError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("db error: {}", e)),
            AppError::NotFound(s) => (StatusCode::NOT_FOUND, format!("not found: {}", s)),
            AppError::InternalError(s) => (StatusCode::INTERNAL_SERVER_ERROR, format!("internal error: {}", s)),
            AppError::BadRequest(s) => (StatusCode::BAD_REQUEST, format!("bad request: {}", s)),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::TemplateError(s) => (StatusCode::INTERNAL_SERVER_ERROR, format!("template error: {}", s)),
        };

        (code, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::DbError(e)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError::InternalError(value.to_string())
    }
}

impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        AppError::TemplateError(value.to_string())
    }
}
