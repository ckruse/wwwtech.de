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

pub fn pictures_atom_uri() -> String {
    let mut uri = pictures_uri();
    uri.push_str(".atom");
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
