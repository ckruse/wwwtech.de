use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::http::HeaderValue;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use reqwest::header::{CACHE_CONTROL, EXPIRES};

use chrono::{Duration, Utc};

pub struct Caching;

impl<S, B> Transform<S> for Caching
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CachingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CachingMiddleware { service })
    }
}

pub struct CachingMiddleware<S> {
    service: S,
}

impl<S, B> Service for CachingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let dt = Utc::now() + Duration::days(365);
            let mut res = fut.await?;
            let headers = res.headers_mut();
            let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

            headers.append(EXPIRES, HeaderValue::from_str(&value).unwrap());
            headers.append(CACHE_CONTROL, HeaderValue::from_static("public,max-age=31536000"));
            return Ok(res);
        })
    }
}
