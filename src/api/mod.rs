use actix_http::error::ErrorInternalServerError;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::{
    extractors::basic::BasicAuth, extractors::AuthenticationError, headers::www_authenticate::basic::Basic,
    middleware::HttpAuthentication,
};

use crate::{session::actions, DbPool};

pub mod likes;
pub mod notes;
pub mod pictures;

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    let challenge = Basic::with_realm("API access");
    let user_id = credentials.user_id().clone();

    let pool = match req.app_data::<web::Data<DbPool>>().map(|data| data.clone()) {
        Some(v) => v,
        None => return Err(ErrorInternalServerError("no pool found")),
    };

    if credentials.password().is_none() {
        return Err(AuthenticationError::new(challenge).into());
    }

    let password = credentials.password().unwrap();
    let user_result = web::block(move || {
        let conn = pool.get()?;
        actions::get_author_by_email(&user_id, &conn)
    })
    .await;

    let author = match user_result {
        Ok(author) => author,
        _ => return Err(AuthenticationError::new(challenge).into()),
    };

    if actions::verify_password(&author, &password) {
        Ok(req)
    } else {
        Err(AuthenticationError::new(challenge).into())
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    let middleware = HttpAuthentication::basic(validator);

    cfg.service(
        web::scope("/api")
            .wrap(middleware)
            .service(notes::index)
            .service(notes::create)
            .service(notes::update)
            .service(notes::delete)
            .service(pictures::index)
            .service(pictures::create)
            .service(pictures::update)
            .service(pictures::delete)
            .service(likes::index)
            .service(likes::create)
            .service(likes::update)
            .service(likes::delete),
    );
}