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
