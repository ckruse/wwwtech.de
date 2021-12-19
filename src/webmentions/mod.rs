use actix_web::{error, post, web, Error, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use url::Url;
use validator::Validate;

use crate::{uri_helpers::root_uri, DbError, DbPool};

use self::actions::{create_mention, mention_exists, target_exists};

pub mod actions;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(receive_webmention);
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct MentionValues {
    #[validate(url)]
    pub source: String,
    #[validate(url)]
    pub target: String,
}

#[post("/webmentions")]
pub async fn receive_webmention(
    pool: web::Data<DbPool>,
    values: web::Form<MentionValues>,
) -> Result<HttpResponse, Error> {
    let root_url = Url::parse(&root_uri()).unwrap();
    let source_url =
        Url::parse(&values.source).map_err(|_| error::ErrorInternalServerError(format!("source url invalid")))?;
    let target_url =
        Url::parse(&values.target).map_err(|_| error::ErrorInternalServerError(format!("target url invalid")))?;

    if target_url.host_str() != root_url.host_str() {
        return Err(error::ErrorInternalServerError(format!("target url invalid")));
    }

    let target_url_ = target_url.clone();
    let pool_ = pool.clone();
    let (exists, object_type, id) = web::block(move || -> Result<Option<(bool, String, i32)>, DbError> {
        let conn = pool_.get()?;
        Ok(target_exists(&target_url_, &conn))
    })
    .await
    .unwrap_or_else(|_| Some((false, "".to_owned(), 0)))
    .unwrap();

    if !exists {
        return Err(error::ErrorInternalServerError(format!("target url invalid")));
    }

    let resp = reqwest::get(source_url.to_string())
        .await
        .map_err(|_| error::ErrorInternalServerError(format!("source url invalid")))?
        .text()
        .await
        .map_err(|_| error::ErrorInternalServerError(format!("source url invalid")))?;

    if !resp.contains(&target_url.to_string()) {
        return Err(error::ErrorInternalServerError(format!("source url invalid")));
    }

    let pool_ = pool.clone();
    let target_url_ = target_url.clone();
    let source_url_ = source_url.clone();
    let mention_exists = web::block(move || -> Result<bool, DbError> {
        let conn = pool_.get()?;
        Ok(mention_exists(
            &target_url_.to_string(),
            &source_url_.to_string(),
            &conn,
        ))
    })
    .await
    .unwrap_or_else(|_| false);

    if mention_exists {
        return Ok(HttpResponse::Ok().content_type("text/plain; charset=utf-8").body("OK"));
    }

    web::block(move || {
        let conn = pool.get()?;
        create_mention(
            &source_url.to_string(),
            &target_url.to_string(),
            &object_type,
            id,
            &conn,
        )
    })
    .await?;

    Ok(HttpResponse::Ok().content_type("text/plain; charset=utf-8").body("OK"))
}
