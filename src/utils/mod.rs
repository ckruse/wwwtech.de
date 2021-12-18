use std::env;

use askama::Result;
use chrono::{naive::NaiveDateTime, Utc};
use pulldown_cmark::{html, Options, Parser};

pub mod paging;

pub fn base_path() -> String {
    env::var("BASE_PATH").unwrap_or(env::var("CARGO_MANIFEST_DIR").unwrap())
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

pub fn link_class_by_type(link_type: &str) -> Result<String> {
    match link_type {
        "reply" => Ok("u-in-reply-to".to_owned()),
        "repost" => Ok("u-repost-of".to_owned()),
        _ => Ok("".to_owned()),
    }
}
