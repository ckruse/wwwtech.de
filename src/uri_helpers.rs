use std::env;

use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result};

#[cfg(debug_assertions)]
pub fn asset_base_uri() -> String {
    "http://localhost:8081/".to_owned()
}

#[cfg(not(debug_assertions))]
pub fn asset_base_uri() -> String {
    concat!(env!("BASE_URI"), "static/").to_owned()
}

pub fn asset_uri(asset: &str) -> String {
    let mut base = asset_base_uri();
    if !base.ends_with("/") {
        base.push_str("/");
    }

    base.push_str(asset.trim_start_matches("/"));
    base
}

pub fn root_uri() -> String {
    env::var("BASE_URI").unwrap_or("http://localhost:8080/".to_owned())
}

pub fn page_uri(page: &str) -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str(page.trim_start_matches("/"));
    uri
}

pub fn tera_asset_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let asset = match args.get("asset") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(format!(
                    "Function `asset_uri` received asset={} but `asset` can only be a string",
                    val
                )));
            }
        },
        None => {
            return Err(Error::msg(
                "Function `asset_uri` didn't receive an `asset` argument",
            ))
        }
    };

    Ok(to_value(asset_uri(&asset)).unwrap())
}

pub fn tera_root_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(root_uri()).unwrap())
}

pub fn tera_page_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let page = match args.get("page") {
        Some(val) => match from_value::<String>(val.clone()) {
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

    Ok(to_value(page_uri(&page)).unwrap())
}
