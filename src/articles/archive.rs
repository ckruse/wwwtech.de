use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::header,
    response::IntoResponse,
};
#[cfg(not(debug_assertions))]
use chrono::Duration;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

use super::{actions, PER_PAGE};
use crate::{
    errors::AppError, models::Article, uri_helpers::*, utils as filters, utils::paging::*, AppState, AuthSession,
};

#[derive(Template)]
#[template(path = "articles/archive_month.html.jinja")]
pub struct Month<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    paging: Paging,

    year: i32,
    short_month: String,
    date: NaiveDateTime,
    articles: Vec<Article>,
    index: bool,
    atom: bool,
}

pub async fn monthly_view(
    auth: AuthSession,
    State(state): State<AppState>,
    Path((year, month_str)): Path<(i32, String)>,
    Query(page): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let p = get_page(&page);
    let logged_in = auth.user.is_some();
    let month =
        filters::month_abbr_to_month_num(&month_str).map_err(|_e| AppError::NotFound("could not find".to_owned()))?;
    let mut conn = state.pool.acquire().await?;

    let articles =
        actions::get_articles_for_year_and_month(year, month, PER_PAGE, p * PER_PAGE, !logged_in, &mut conn).await?;
    let count = actions::count_articles_for_year_and_month(year, month, !logged_in, &mut conn).await?;

    let paging = get_paging(count, p, PER_PAGE);

    let dt = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| AppError::NotFound("could not find".to_owned()))?;
    let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let date = NaiveDateTime::new(dt, t);

    #[cfg(debug_assertions)]
    let headers = {
        let dt = Utc::now();
        let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let duration_seconds = "public,max-age=0".to_owned();

        [(header::EXPIRES, value), (header::CACHE_CONTROL, duration_seconds)]
    };

    #[cfg(not(debug_assertions))]
    let headers = {
        let duration = Duration::days(30);
        let dt = Utc::now() + duration;
        let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let duration_seconds = format!("public,max-age={}", duration.num_seconds());

        [(header::EXPIRES, value), (header::CACHE_CONTROL, duration_seconds)]
    };

    Ok((
        headers,
        Month {
            lang: "en",
            title: Some(format!("Articles in {}", date.format("%B, %Y"))),
            page_type: None,
            page_image: None,
            body_id: None,
            paging,
            logged_in,
            year,
            date,
            short_month: month_str,
            articles,
            index: true,
            atom: false,
        },
    ))
}

#[derive(Template)]
#[template(path = "articles/archive_year.html.jinja")]
pub struct Year<'a> {
    lang: &'a str,
    title: Option<String>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    paging: Paging,

    year: i32,
    articles: Vec<Article>,
    index: bool,
    atom: bool,
}

pub async fn yearly_view(
    auth: AuthSession,
    State(state): State<AppState>,
    Path(year): Path<i32>,
    Query(page): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let p = get_page(&page);
    let logged_in = auth.user.is_some();
    let mut conn = state.pool.acquire().await?;

    let articles = actions::get_articles_for_year(year, PER_PAGE, p * PER_PAGE, !logged_in, &mut conn).await?;
    let count = actions::count_articles_for_year(year, !logged_in, &mut conn).await?;

    let paging = get_paging(count, p, PER_PAGE);

    #[cfg(debug_assertions)]
    let headers = {
        let dt = Utc::now();
        let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let duration_seconds = "public,max-age=0".to_owned();

        [(header::EXPIRES, value), (header::CACHE_CONTROL, duration_seconds)]
    };

    #[cfg(not(debug_assertions))]
    let headers = {
        let duration = Duration::days(30);
        let dt = Utc::now() + duration;
        let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let duration_seconds = format!("public,max-age={}", duration.num_seconds());

        [(header::EXPIRES, value), (header::CACHE_CONTROL, duration_seconds)]
    };

    Ok((
        headers,
        Year {
            lang: "en",
            title: Some(format!("Articles in {}", year)),
            page_type: None,
            page_image: None,
            body_id: None,
            logged_in,
            paging,
            year,
            articles,
            index: true,
            atom: false,
        },
    ))
}
