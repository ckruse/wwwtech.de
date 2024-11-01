use axum::extract::State;
use axum::http::{self, header};
use axum::response::Response;
use chrono::Duration;
#[cfg(not(debug_assertions))]
use chrono::Utc;

use crate::uri_helpers::webmentions_endpoint_uri;

pub async fn webmention_middleware<B>(mut response: Response<B>) -> Response<B> {
    if let Ok(value) = http::HeaderValue::from_str(&format!("<{}>; rel=\"webmention\"", webmentions_endpoint_uri())) {
        response.headers_mut().insert(header::LINK, value);
    }

    response
}

#[allow(unused_mut, unused_variables)]
pub async fn caching_middleware<B>(State(duration): State<Duration>, mut response: Response<B>) -> Response<B> {
    #[cfg(not(debug_assertions))]
    {
        let dt = Utc::now() + duration;
        let Ok(value) = http::HeaderValue::from_str(&dt.to_rfc2822()) else {
            return response;
        };

        let Ok(duration_seconds) = http::HeaderValue::from_str(&format!("public,max-age={}", duration.num_seconds()))
        else {
            return response;
        };

        let headers = response.headers_mut();
        headers.insert(header::EXPIRES, value);
        headers.insert(header::CACHE_CONTROL, duration_seconds);
    }

    response
}
