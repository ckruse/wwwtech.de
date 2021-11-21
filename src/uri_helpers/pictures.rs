use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result};

use crate::models::Picture;
use crate::uri_helpers::root_uri;

pub fn pictures_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("pictures");
    uri
}

pub fn picture_uri(picture: &Picture) -> String {
    let mut uri = pictures_uri();
    uri.push_str("/");
    uri.push_str(&picture.id.to_string());

    uri
}

pub fn picture_img_uri(picture: &Picture) -> String {
    let mut uri = picture_uri(picture);

    let suffix = match picture.image_content_type.as_str() {
        "image/png" => ".png",
        "image/jpg" => ".jpg",
        "image/jpeg" => ".jpg",
        "image/gif" => ".gif",
        _ => ".unknown",
    };

    uri.push_str(suffix);

    uri
}

pub fn edit_picture_uri(picture: &Picture) -> String {
    let mut uri = pictures_uri();
    uri.push_str("/");
    uri.push_str(&picture.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn new_picture_uri() -> String {
    let mut uri = pictures_uri();
    uri.push_str("/new");

    uri
}

pub fn tera_pictures_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(pictures_uri()).unwrap())
}

pub fn tera_new_picture_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(new_picture_uri()).unwrap())
}

pub fn tera_picture_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let picture = match args.get("picture") {
        Some(val) => match from_value::<Picture>(val.clone()) {
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

    Ok(to_value(picture_uri(&picture)).unwrap())
}

pub fn tera_picture_img_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let picture = match args.get("picture") {
        Some(val) => match from_value::<Picture>(val.clone()) {
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

    Ok(to_value(picture_img_uri(&picture)).unwrap())
}

pub fn tera_edit_picture_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let picture = match args.get("picture") {
        Some(val) => match from_value::<Picture>(val.clone()) {
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

    Ok(to_value(edit_picture_uri(&picture)).unwrap())
}
