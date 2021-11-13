use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result};

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

pub fn tera_notes_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(notes_uri()).unwrap())
}

pub fn tera_new_note_uri(_args: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(new_note_uri()).unwrap())
}

pub fn tera_note_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let note = match args.get("note") {
        Some(val) => match from_value::<Note>(val.clone()) {
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

    Ok(to_value(note_uri(&note)).unwrap())
}

pub fn tera_edit_note_uri(args: &HashMap<String, Value>) -> Result<Value> {
    let note = match args.get("note") {
        Some(val) => match from_value::<Note>(val.clone()) {
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

    Ok(to_value(edit_note_uri(&note)).unwrap())
}
