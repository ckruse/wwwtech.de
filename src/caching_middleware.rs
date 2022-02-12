use std::pin::Pin;

#[cfg(not(debug_assertions))]
use actix_http::header::HeaderValue;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
#[cfg(not(debug_assertions))]
use reqwest::header::{CACHE_CONTROL, EXPIRES};

use chrono::Duration;
#[cfg(not(debug_assertions))]
use chrono::Utc;

pub struct Caching {
    pub duration: Duration,
}

impl<S, B> Transform<S, ServiceRequest> for Caching
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = CachingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CachingMiddleware {
            service,
            duration: self.duration,
        })
    }
}

pub struct CachingMiddleware<S> {
    service: S,
    #[allow(dead_code)]
    duration: Duration,
}

impl<S, B> Service<ServiceRequest> for CachingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        #[cfg(not(debug_assertions))]
        let duration = self.duration.clone();

        #[cfg(not(debug_assertions))]
        return Box::pin(async move {
            let dt = Utc::now() + duration;
            let mut res = fut.await?;
            let headers = res.headers_mut();
            let value = dt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
            let duration_seconds = format!("public,max-age={}", duration.num_seconds());

            headers.append(EXPIRES, HeaderValue::from_str(&value).unwrap());
            headers.append(CACHE_CONTROL, HeaderValue::from_str(&duration_seconds).unwrap());
            return Ok(res);
        });

        #[cfg(debug_assertions)]
        return Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        });
    }
}
