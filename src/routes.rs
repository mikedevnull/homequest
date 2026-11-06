use crate::auth::{self, AuthService, DefaultAuthService};
use actix_files as fs;
use actix_identity::Identity;
use actix_web::{HttpResponse, Responder, get, http, web};
use sqlx::SqlitePool;
use tera::{Context, Tera};

#[get("/")]
async fn index(
    tera: web::Data<Tera>,
    auth_service: web::Data<DefaultAuthService>,
    user: Option<Identity>,
) -> impl Responder {
    if user.is_none() {
        return HttpResponse::Found()
            .append_header((http::header::LOCATION, "/login"))
            .finish();
    }
    let mut context = Context::new();
    auth_service.inject_user_info(&mut context, user).await;
    match tera.render("pages/index.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig, db: SqlitePool) {
    cfg.default_service(web::to(|| HttpResponse::NotFound()));
    cfg.service(index);
    cfg.configure(|cfg| auth::configure(cfg, db));
    cfg.service(fs::Files::new("/static", "./templates/static").show_files_listing());
}
