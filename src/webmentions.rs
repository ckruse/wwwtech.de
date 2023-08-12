use axum::{
    debug_handler,
    extract::{Form, State},
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use url::Url;
use visdom::{types::IAttrValue, Vis};

use self::actions::{create_mention, mention_exists, target_exists, ObjectType};
use crate::{errors::AppError, uri_helpers::root_uri, AppRouter, AppState};

pub mod actions;
pub mod send;

mod mail_sender;

pub fn configure(app: AppRouter) -> AppRouter {
    app.route("/webmentions", post(receive_webmention))
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MentionValues {
    pub source: String,
    pub target: String,
}

#[debug_handler]
pub async fn receive_webmention(
    State(state): State<AppState>,
    Form(values): Form<MentionValues>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.pool.acquire().await?;
    let root_url = Url::parse(&root_uri()).unwrap();
    let source_url =
        Url::parse(&values.source).map_err(|_| AppError::InternalError("source url invalid".to_owned()))?;
    let target_url =
        Url::parse(&values.target).map_err(|_| AppError::InternalError("target url invalid".to_owned()))?;

    if target_url.host_str() != root_url.host_str() {
        return Err(AppError::BadRequest("target url invalid".to_owned()));
    }

    let (object_type, id) = target_exists(&target_url, &mut conn)
        .await
        .unwrap_or((ObjectType::Article, 0));

    let surl = source_url.to_string();
    let body = reqwest::get(surl)
        .await
        .map_err(|e| AppError::InternalError(format!("request error: {}", e)))?
        .text()
        .await
        .map_err(|e| AppError::InternalError(format!("request error: {}", e)))?;

    if !body.contains(&target_url.to_string()) {
        return Err(AppError::BadRequest("source url invalid".to_owned()));
    }

    let mention_exists = mention_exists(source_url.as_ref(), target_url.as_ref(), &mut conn).await;

    if mention_exists {
        return Ok("OK");
    }

    let (title, author) = {
        let tree = Vis::load(&body).map_err(|_| AppError::BadRequest("could not parse source document".to_owned()))?;
        let title = tree.find("title").text();
        let author = match tree.find("meta[name=author]").attr("content") {
            Some(IAttrValue::Value(author, _)) => author,
            _ => "unknown".to_owned(),
        };

        (title, author)
    };

    let mention = create_mention(
        source_url.to_string(),
        target_url.to_string(),
        object_type,
        id,
        author,
        title,
        &mut conn,
    )
    .await?;

    if source_url.host() != root_url.host() {
        tokio::task::spawn_blocking(move || mail_sender::send_mail(mention));
    }

    Ok("OK")
}
