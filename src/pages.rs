use actix_web::{error, get, web, Error, HttpResponse, Result};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(software).service(about).service(more);
}

#[get("/")]
pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("pages/index.html.tera", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[get("/software")]
pub async fn software(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("pages/software.html.tera", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[get("/about")]
pub async fn about(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("pages/about.html.tera", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

#[get("/more")]
pub async fn more(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("pages/more.html.tera", &tera::Context::new())
        .map_err(|e| error::ErrorInternalServerError(format!("Template error: {}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}
