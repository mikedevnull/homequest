use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/login")]
async fn login(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    match tera.render("pages/login.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}