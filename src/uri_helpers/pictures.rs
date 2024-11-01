use crate::models::Picture;
use crate::uri_helpers::root_uri;
use crate::utils::content_type_from_suffix;

pub fn pictures_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with('/') {
        uri.push('/');
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
    uri.push('/');
    uri.push_str(&picture.id.to_string());

    uri
}

pub fn picture_img_uri(picture: &Picture, picture_type: Option<&str>) -> String {
    let mut uri = picture_uri(picture);
    let suffix = content_type_from_suffix(&picture.image_content_type);
    uri.push_str(suffix);

    if let Some(picture_type) = picture_type {
        uri.push_str("?type=");
        uri.push_str(picture_type);
    }

    uri
}

pub fn edit_picture_uri(picture: &Picture) -> String {
    let mut uri = pictures_uri();
    uri.push('/');
    uri.push_str(&picture.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn new_picture_uri() -> String {
    let mut uri = pictures_uri();
    uri.push_str("/new");

    uri
}

pub fn delete_picture_uri(picture: &Picture) -> String {
    let mut uri = pictures_uri();
    uri.push('/');
    uri.push_str(&picture.id.to_string());
    uri.push_str("/delete");

    uri
}
