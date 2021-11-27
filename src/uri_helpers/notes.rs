use crate::models::Note;
use crate::uri_helpers::root_uri;

pub fn notes_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("notes");
    uri
}

pub fn note_uri(note: &Note) -> String {
    let mut uri = notes_uri();
    uri.push_str("/");
    uri.push_str(&note.id.to_string());

    uri
}

pub fn edit_note_uri(note: &Note) -> String {
    let mut uri = notes_uri();
    uri.push_str("/");
    uri.push_str(&note.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn new_note_uri() -> String {
    let mut uri = notes_uri();
    uri.push_str("/new");

    uri
}
