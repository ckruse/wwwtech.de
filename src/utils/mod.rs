use std::env;

use pulldown_cmark::{html, Options, Parser};
use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result, Tera};

use chrono::{naive::NaiveDateTime, Utc};

use crate::uri_helpers;

pub mod paging;

pub fn base_path() -> String {
    env::var("BASE_PATH").unwrap_or(env::var("CARGO_MANIFEST_DIR").unwrap())
}

pub fn static_path() -> String {
    let mut str = base_path();
    str.push_str("/static/");

    str
}

pub fn markdown2html(md: &String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn markdown2html_tera(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let md = match from_value::<String>(value.clone()) {
        Ok(v) => v,
        Err(_) => return Err(Error::msg("Filter `markdown2html` can only work on strings")),
    };

    Ok(to_value(markdown2html(&md)).unwrap())
}

pub fn time_ago_in_words(time: &NaiveDateTime) -> String {
    let formatter = timeago::Formatter::new();
    let now = Utc::now().naive_utc();

    let duration = now.signed_duration_since(time.clone());

    formatter.convert(duration.to_std().unwrap())
}

pub fn time_ago_tera(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let time = match from_value::<NaiveDateTime>(value.clone()) {
        Ok(v) => v,
        Err(_) => return Err(Error::msg("Filter `time_ago` can only work on NaiveDateTime")),
    };

    Ok(to_value(time_ago_in_words(&time)).unwrap())
}

pub fn register_tera_functions(tera: &mut Tera) {
    tera.register_filter("markdown2html", markdown2html_tera);
    tera.register_filter("time_ago", time_ago_tera);

    tera.register_function("asset_uri", uri_helpers::tera_asset_uri);
    tera.register_function("root_uri", uri_helpers::tera_root_uri);
    tera.register_function("page_uri", uri_helpers::tera_page_uri);

    tera.register_function("articles_uri", uri_helpers::tera_articles_uri);
    tera.register_function("article_uri", uri_helpers::tera_article_uri);
    tera.register_function("edit_article_uri", uri_helpers::tera_edit_article_uri);
    tera.register_function("new_article_uri", uri_helpers::tera_new_article_uri);

    tera.register_function("notes_uri", uri_helpers::tera_notes_uri);
    tera.register_function("note_uri", uri_helpers::tera_note_uri);
    tera.register_function("edit_note_uri", uri_helpers::tera_edit_note_uri);
    tera.register_function("new_note_uri", uri_helpers::tera_new_note_uri);

    tera.register_function("pictures_uri", uri_helpers::tera_pictures_uri);
    tera.register_function("picture_uri", uri_helpers::tera_picture_uri);
    tera.register_function("edit_picture_uri", uri_helpers::tera_edit_picture_uri);
    tera.register_function("new_picture_uri", uri_helpers::tera_new_picture_uri);

    tera.register_function("likes_uri", uri_helpers::tera_likes_uri);
    tera.register_function("like_uri", uri_helpers::tera_like_uri);
    tera.register_function("edit_like_uri", uri_helpers::tera_edit_like_uri);
    tera.register_function("new_like_uri", uri_helpers::tera_new_like_uri);
}
