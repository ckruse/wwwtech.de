use std::env;

pub mod articles;
pub mod deafies;
pub mod likes;
pub mod notes;
pub mod pictures;

pub use articles::*;
pub use deafies::*;
pub use likes::*;
pub use notes::*;
pub use pictures::*;

const ASSET_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn asset_base_uri() -> String {
    let mut base = env::var("BASE_URI").expect("BASE_URI is not set");
    base.push_str("static/");
    base
}

pub fn asset_uri(asset: &str) -> String {
    let mut base = asset_base_uri();
    if !base.ends_with("/") {
        base.push_str("/");
    }

    base.push_str(asset.trim_start_matches("/"));
    base.push_str("?");
    base.push_str(ASSET_VERSION);

    base
}

pub fn root_uri() -> String {
    env::var("BASE_URI").expect("BASE_URI is not set")
}

pub fn page_uri(page: &str) -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str(page.trim_start_matches("/"));
    uri
}

pub fn whatsnew_atom_uri() -> String {
    page_uri("whatsnew.atom")
}

pub fn login_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("login");

    uri
}

pub fn logout_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("logout");

    uri
}

pub fn webmentions_endpoint_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("webmentions");
    uri
}
