use actix_http::{Error, Result};
use actix_multipart::{Multipart, MultipartError};
use actix_web::web;
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
        // println!("field: {:?}", field.content_disposition());

        let content_disposition = field.content_disposition().ok_or_else(|| MultipartError::Boundary)?;

        if content_disposition.get_name().is_none() {
            continue;
        }

        let filename = content_disposition.get_filename();
        let name = content_disposition.get_name().unwrap();

        if let Some(filename) = filename {
            let mut file = web::block(|| tempfile()).await?;

            while let Some(chunk) = field.try_next().await? {
                // filesystem operations are blocking, we have to use threadpool
                file = web::block(move || file.write_all(&chunk).map(|_| file)).await?;
            }

            params.insert(name.to_owned(), MultipartField::File(filename.to_owned(), file));
        } else {
            let mut data = String::new();

            while let Some(chunk) = field.try_next().await? {
                match std::str::from_utf8(&chunk.to_vec()) {
                    Ok(value) => data += value,
                    Err(_) => continue,
                }
            }

            params.insert(name.to_owned(), MultipartField::Form(data));
        }
    }

    Ok(params)
}
