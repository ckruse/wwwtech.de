use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result};

use crate::models::Like;
use crate::uri_helpers::root_uri;

pub fn likes_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("likes");
    uri
}

pub fn like_uri(like: &Like) -> String {
    let mut uri = likes_uri();
    uri.push_str("/");
    uri.push_str(&like.id.to_string());

    uri
}

pub fn edit_like_uri(like: &Like) -> String {
    let mut uri = likes_uri();
    uri.push_str("/");
    uri.push_str(&like.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn new_like_uri() -> String {
    let mut uri = likes_uri();
    uri.push_str("/new");

    uri
}

pub fn tera_likes_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(likes_uri()).unwrap())
}

pub fn tera_new_like_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(new_like_uri()).unwrap())
}

pub fn tera_like_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let like = match args.get("like") {
        Some(val) => match from_value::<Like>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(format!(
                    "Function `page_uri` received page={} but `page` can only be a string",
                    val
                )));
            }
        },
        None => {
            return Err(Error::msg(
                "Function `page_uri` didn't receive a `page` argument",
            ))
        }
    };

    Ok(to_value(like_uri(&like)).unwrap())
}

pub fn tera_edit_like_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let like = match args.get("like") {
        Some(val) => match from_value::<Like>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(format!(
                    "Function `page_uri` received page={} but `page` can only be a string",
                    val
                )));
            }
        },
        None => {
            return Err(Error::msg(
                "Function `page_uri` didn't receive a `page` argument",
            ))
        }
    };

    Ok(to_value(edit_like_uri(&like)).unwrap())
}
