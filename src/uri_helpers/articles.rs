use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result};

use crate::models::Article;
use crate::uri_helpers::root_uri;

pub fn articles_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("articles");
    uri
}

pub fn article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());

    uri
}

pub fn edit_article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn new_article_uri() -> String {
    let mut uri = articles_uri();
    uri.push_str("/new");

    uri
}

pub fn tera_articles_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(articles_uri()).unwrap())
}

pub fn tera_new_article_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(new_article_uri()).unwrap())
}

pub fn tera_article_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let article = match args.get("article") {
        Some(val) => match from_value::<Article>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(format!(
                    "Function `page_uri` received page={} but `page` can only be a string",
                    val
                )));
            }
        },
        None => return Err(Error::msg("Function `page_uri` didn't receive a `page` argument")),
    };

    Ok(to_value(article_uri(&article)).unwrap())
}

pub fn tera_edit_article_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let article = match args.get("article") {
        Some(val) => match from_value::<Article>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(format!(
                    "Function `page_uri` received page={} but `page` can only be a string",
                    val
                )));
            }
        },
        None => return Err(Error::msg("Function `page_uri` didn't receive a `page` argument")),
    };

    Ok(to_value(edit_article_uri(&article)).unwrap())
}
