use crate::{models::Like, uri_helpers::root_uri};

pub fn likes_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with('/') {
        uri.push('/');
    }

    uri.push_str("likes");
    uri
}

pub fn likes_atom_uri() -> String {
    let mut uri = likes_uri();
    uri.push_str(".atom");
    uri
}

pub fn like_uri(like: &Like) -> String {
    let mut uri = likes_uri();
    uri.push('/');
    uri.push_str(&like.id.to_string());

    uri
}

pub fn edit_like_uri(like: &Like) -> String {
    let mut uri = likes_uri();
    uri.push('/');
    uri.push_str(&like.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn delete_like_uri(like: &Like) -> String {
    let mut uri = likes_uri();
    uri.push('/');
    uri.push_str(&like.id.to_string());
    uri.push_str("/delete");

    uri
}

pub fn new_like_uri() -> String {
    let mut uri = likes_uri();
    uri.push_str("/new");

    uri
}
