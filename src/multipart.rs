use actix_multipart::Multipart;
use actix_web::{web, Error, Result};
use futures_util::TryStreamExt as _;
use std::{collections::HashMap, fs::File, io::Write};
use tempfile::tempfile;

#[derive(Debug)]
pub enum MultipartField {
    Form(String),
    File(String, File),
}

pub async fn parse_multipart(payload: &mut Multipart) -> Result<HashMap<String, MultipartField>, Error> {
    let mut params: HashMap<String, MultipartField> = HashMap::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition().clone();
        let name = content_disposition.get_name();
        let filename = content_disposition.get_filename();

        if name.is_none() || name == Some("") {
            continue;
        }

        let name = name.unwrap();
        let (filename, is_file) = match filename {
            Some(filename) => (filename, filename.is_empty()),
            None => ("", false),
        };

        if is_file {
            let mut file = web::block(tempfile).await??;

            while let Some(chunk) = field.try_next().await? {
                // filesystem operations are blocking, we have to use threadpool
                file = web::block(move || file.write_all(&chunk).map(|_| file)).await??;
            }

            params.insert(name.to_owned(), MultipartField::File(filename.to_owned(), file));
        } else {
            let mut data = String::new();

            while let Some(chunk) = field.try_next().await? {
                match std::str::from_utf8(&chunk) {
                    Ok(value) => data += value,
                    Err(_) => continue,
                }
            }

            params.insert(name.to_owned(), MultipartField::Form(data));
        }
    }

    Ok(params)
}

pub fn get_file(params: &HashMap<String, MultipartField>) -> Option<(String, &File)> {
    let field = params.get("picture");

    if let Some(MultipartField::File(filename, file)) = field {
        Some((filename.clone(), file))
    } else {
        None
    }
}
