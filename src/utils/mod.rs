use std::env;

use askama::Result;
use chrono::{naive::NaiveDateTime, Duration, NaiveDate, Utc};
use pulldown_cmark::{html, Options, Parser};

use crate::models::Note;

pub mod paging;

pub static MONTHS: [&'static str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

pub fn base_path() -> String {
    env::var("BASE_PATH").expect("env variable BASE_PATH not set")
}

pub fn image_base_path() -> String {
    env::var("IMAGE_BASE_PATH").expect("env variable IMAGE_BASE_PATH not set")
}

pub fn static_path() -> String {
    let mut str = base_path();
    str.push_str("/static/");

    str
}

pub fn markdown2html(md: &String) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

pub fn time_ago_in_words(time: &NaiveDateTime) -> Result<String> {
    let formatter = timeago::Formatter::new();
    let now = Utc::now().naive_utc();

    let duration = now.signed_duration_since(time.clone());

    Ok(formatter.convert(duration.to_std().unwrap()))
}

pub fn date_format(date: &NaiveDateTime, format: &str) -> Result<String> {
    Ok(date.format(format).to_string())
}

pub fn link_class_by_type(note: &Note) -> Result<String> {
    let start_date: NaiveDate = NaiveDate::from_ymd(2017, 1, 19);

    match note.note_type.as_ref() {
        "reply" => Ok("u-in-reply-to".to_owned()),
        "repost" => Ok("u-repost-of".to_owned()),
        _ => {
            if note.inserted_at.date() < start_date {
                Ok("u-in-reply-to".to_owned())
            } else {
                Ok("".to_owned())
            }
        }
    }
}

pub fn entry_class_by_type(entry_type: &str) -> Result<String> {
    match entry_type {
        "reply" => Ok("h-as-reply".to_owned()),
        "repost" => Ok("p-repost".to_owned()),
        "like" => Ok("p-like".to_owned()),
        "favorite" => Ok("p-favorite".to_owned()),
        "tag" => Ok("p-tag".to_owned()),
        "bookmark" => Ok("p-bookmark".to_owned()),
        _ => Ok("".to_owned()),
    }
}

pub fn date_list_format(date: &NaiveDateTime) -> Result<String> {
    let today = Utc::now().naive_utc().date();
    let day = date.date();

    if day == today {
        return Ok("today".to_owned());
    } else if day == (today - Duration::days(1)) {
        return Ok("yesterday".to_owned());
    }

    Ok(date.format("%Y-%m-%d").to_string())
}
