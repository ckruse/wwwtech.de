use crate::models::Deafie;
use crate::uri_helpers::root_uri;
use crate::utils::content_type_from_suffix;

pub fn deafies_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with('/') {
        uri.push('/');
    }

    uri.push_str("the-life-of-alfons");
    uri
}

pub fn deafies_atom_uri() -> String {
    let mut uri = deafies_uri();
    uri.push_str(".atom");
    uri
}

pub fn deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push('/');
    uri.push_str(&deafie.slug);

    uri
}

pub fn deafie_img_uri(deafie: &Deafie, picture_type: Option<&str>) -> String {
    let mut uri = deafie_uri(deafie);

    let suffix = match &deafie.image_content_type {
        Some(name) => content_type_from_suffix(name),
        None => ".unknown",
    };

    uri.push_str(suffix);

    if let Some(picture_type) = picture_type {
        uri.push_str("?type=");
        uri.push_str(picture_type);
    }

    uri
}

pub fn edit_deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push('/');
    uri.push_str(&deafie.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn update_deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push('/');
    uri.push_str(&deafie.id.to_string());

    uri
}

pub fn delete_deafie_uri(deafie: &Deafie) -> String {
    let mut uri = deafies_uri();
    uri.push('/');
    uri.push_str(&deafie.id.to_string());
    uri.push_str("/delete");

    uri
}

pub fn new_deafie_uri() -> String {
    let mut uri = deafies_uri();
    uri.push_str("/new");

    uri
}
