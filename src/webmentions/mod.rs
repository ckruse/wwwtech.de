use actix_web::{error, post, web, Error, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use url::Url;
use visdom::{types::IAttrValue, Vis};

use crate::{uri_helpers::root_uri, DbError, DbPool};

use self::actions::{create_mention, mention_exists, target_exists};

pub mod actions;
pub mod send;

mod mail_sender;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(receive_webmention);
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MentionValues {
    pub source: String,
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
        return Err(error::ErrorBadRequest("target url invalid"));
    }

    let target_url_ = target_url.clone();
    let pool_ = pool.clone();
    let (object_type, id) = web::block(move || -> Result<Option<(String, i32)>, DbError> {
        let mut conn = pool_.get()?;
        Ok(target_exists(&target_url_, &mut conn))
    })
    .await?
    .map_err(|_| error::ErrorBadRequest("not a valid webention endpoint"))?
    .unwrap_or_else(|| ("".to_owned(), 0));

    let surl = source_url.to_string();
    let body = web::block(move || reqwest::blocking::get(surl)?.text())
        .await?
        .map_err(|e| error::ErrorInternalServerError(format!("request error: {}", e)))?;

    if !body.contains(&target_url.to_string()) {
        return Err(error::ErrorBadRequest(format!("source url invalid")));
    }

    let pool_ = pool.clone();
    let target_url_ = target_url.clone();
    let source_url_ = source_url.clone();
    let mention_exists = web::block(move || -> Result<bool, DbError> {
        let mut conn = pool_.get()?;
        Ok(mention_exists(
            &source_url_.to_string(),
            &target_url_.to_string(),
            &mut conn,
        ))
    })
    .await?
    .unwrap_or_else(|_| false);

    if mention_exists {
        return Ok(HttpResponse::Ok().content_type("text/plain; charset=utf-8").body("OK"));
    }

    let tree = Vis::load(&body).map_err(|_| error::ErrorBadRequest("could not parse source document"))?;
    let title = tree.find("title").text().to_owned();
    let author = match tree.find("meta[name=author]").attr("content") {
        Some(IAttrValue::Value(author, _)) => author.clone(),
        _ => "unknown".to_owned(),
    };

    let s_url = source_url.to_string();
    let url = target_url.to_string();
    let mention = web::block(move || {
        let mut conn = pool.get()?;
        create_mention(s_url, url, &object_type, id, author, title, &mut conn)
    })
    .await?
    .map_err(|_| error::ErrorBadRequest("could not create mention"))?;

    if source_url.host() != root_url.host() {
        tokio::task::spawn_blocking(move || mail_sender::send_mail(mention));
    }

    Ok(HttpResponse::Ok().content_type("text/plain; charset=utf-8").body("OK"))
}
