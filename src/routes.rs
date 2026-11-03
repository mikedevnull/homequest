use crate::auth;
use actix_files as fs;
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, get, http, web};
use tera::{Context, Tera};

#[get("/")]
async fn index(tera: web::Data<Tera>, user: Option<Identity>) -> impl Responder {
    if user.is_none() {
        return HttpResponse::Found().append_header((http::header::LOCATION, "/login")).finish();
    }
    let context = Context::new();
    match tera.render("pages/index.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.default_service(web::to(|| HttpResponse::NotFound()));
    cfg.service(index);
    cfg.configure(auth::routes::configure);
    cfg.service(fs::Files::new("/static", "./templates/static").show_files_listing());
}
