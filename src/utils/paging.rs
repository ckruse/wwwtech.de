use actix_web::web;
use serde::{Deserialize, Serialize};

static MAX_PAGE_ENTRIES: i64 = 9;

#[derive(Deserialize)]
pub struct PageParams {
    p: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct Paging {
    pub start: i64,
    pub end: i64,
    pub last_page: i64,
    pub active: i64,
    pub next_page: i64,
    pub prev_page: i64,
}

pub fn get_page(page: &web::Query<PageParams>) -> i64 {
    let mut p = match page.p {
        Some(page) => page,
        None => 0,
    };

    if p < 0 {
        p = 0;
    }

    p
}

pub fn get_paging(count: i64, page: i64, per_page: i64) -> Paging {
    let last_page = count / per_page;
    let mut start = page - (MAX_PAGE_ENTRIES / 2);
    let mut end = page + (MAX_PAGE_ENTRIES / 2);

    if start < 0 {
        end = end - start;
        start = 0;
    }

    if end > last_page {
        let diff = end - last_page;
        end = last_page;

        if start - diff > 0 {
            start -= diff;
        }
    }

    let mut prev_page = page - 1;
    let mut next_page = page + 1;
    if prev_page < 0 {
        prev_page = 0;
    }
    if next_page > last_page {
        next_page = last_page;
    }

    Paging {
        start,
        end,
        last_page,
        active: page,
        next_page,
        prev_page,
    }
}
