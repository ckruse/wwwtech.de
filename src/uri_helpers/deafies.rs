use crate::{models::Deafie, uri_helpers::root_uri};

pub fn deafies_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("deaf-dog-training");
    uri
}

pub fn deafies_atom_uri() -> String {
    let mut uri = deafies_uri();
    uri.push_str(".atom");
    uri
}

pub fn deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push_str("/");
    uri.push_str(&deafie.slug);

    uri
}

pub fn edit_deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push_str("/");
    uri.push_str(&deafie.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn update_deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push_str("/");
    uri.push_str(&deafie.id.to_string());

    uri
}

pub fn delete_deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push_str("/");
    uri.push_str(&deafie.id.to_string());
    uri.push_str("/delete");

    uri
}

pub fn new_deafie_uri() -> String {
    let mut uri = deafies_uri();
    uri.push_str("/new");

    uri
}
