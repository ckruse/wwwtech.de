use actix_identity::Identity;
use actix_web::{error, get, web, Error, HttpResponse, Result};
use askama::Template;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[cfg(not(debug_assertions))]
use actix_web::http::header;
#[cfg(not(debug_assertions))]
use chrono::{Duration, Utc};

use crate::utils::paging::*;
use crate::DbPool;

use super::{actions, PER_PAGE};
use crate::models::Article;

use crate::uri_helpers::*;
use crate::utils as filters;

#[derive(Template)]
#[template(path = "articles/archive_month.html.jinja")]
struct Month<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    paging: &'a Paging,

    year: i32,
    short_month: &'a str,
    date: NaiveDateTime,
    articles: &'a Vec<Article>,
    index: bool,
    atom: bool,
}

#[get("/articles/{year}/{month}")]
pub async fn monthly_view(
    ident: Option<Identity>,
    pool: web::Data<DbPool>,
    path: web::Path<(i32, String)>,
    page: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let p = get_page(&page);
    let logged_in = ident.is_some();
    let (year, month_str) = path.into_inner();
    let month = filters::month_abbr_to_month_num(&month_str).map_err(|_e| error::ErrorNotFound("could not find"))?;

    let pool_ = pool.clone();
    let articles = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_articles_for_year_and_month(year, month, PER_PAGE, p * PER_PAGE, !logged_in, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let mut conn = pool.get()?;
        actions::count_articles_for_year_and_month(year, month, !logged_in, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let dt = NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| error::ErrorNotFound("could not find"))?;
    let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let date = NaiveDateTime::new(dt, t);

    let s = Month {
        lang: "en",
        title: Some(&format!("Articles in {}", date.format("%B, %Y"))),
        page_type: None,
        page_image: None,
        body_id: None,
        paging: &paging,
        logged_in,
        year,
        date,
        short_month: &month_str,
        articles: &articles,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    #[cfg(debug_assertions)]
    let rsp = HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s);

    #[cfg(not(debug_assertions))]
    let rsp = {
        let duration = Duration::days(30);
        let dt = Utc::now() + duration;
        let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let duration_seconds = format!("public,max-age={}", duration.num_seconds());

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .append_header((header::EXPIRES, value))
            .append_header((header::CACHE_CONTROL, duration_seconds))
            .body(s)
    };

    Ok(rsp)
}

#[derive(Template)]
#[template(path = "articles/archive_year.html.jinja")]
struct Year<'a> {
    lang: &'a str,
    title: Option<&'a str>,
    page_type: Option<&'a str>,
    page_image: Option<&'a str>,
    body_id: Option<&'a str>,
    logged_in: bool,

    paging: &'a Paging,

    year: i32,
    articles: &'a Vec<Article>,
    index: bool,
    atom: bool,
}

#[get("/articles/{year}")]
pub async fn yearly_view(
    ident: Option<Identity>,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    page: web::Query<PageParams>,
) -> Result<HttpResponse, Error> {
    let p = get_page(&page);
    let logged_in = ident.is_some();
    let year = path.into_inner();

    let pool_ = pool.clone();
    let articles = web::block(move || {
        let mut conn = pool_.get()?;
        actions::get_articles_for_year(year, PER_PAGE, p * PER_PAGE, !logged_in, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let count = web::block(move || {
        let mut conn = pool.get()?;
        actions::count_articles_for_year(year, !logged_in, &mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    let paging = get_paging(count, p, PER_PAGE);

    let s = Year {
        lang: "en",
        title: Some(&format!("Articles in {}", year)),
        page_type: None,
        page_image: None,
        body_id: None,
        logged_in,
        paging: &paging,
        year,
        articles: &articles,
        index: true,
        atom: false,
    }
    .render()
    .unwrap();

    #[cfg(debug_assertions)]
    let rsp = HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s);

    #[cfg(not(debug_assertions))]
    let rsp = {
        let duration = Duration::days(30);
        let dt = Utc::now() + duration;
        let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let duration_seconds = format!("public,max-age={}", duration.num_seconds());

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .append_header((header::EXPIRES, value))
            .append_header((header::CACHE_CONTROL, duration_seconds))
            .body(s)
    };

    Ok(rsp)
}
