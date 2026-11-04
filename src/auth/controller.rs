use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, get, web};
use tera::{Context, Tera};

use crate::auth::{
    repositories::Group,
    service::{AuthService, Credentials},
};

mod dao {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct LoginForm {
        pub username: String,
        pub password: String,
    }

    #[derive(Deserialize)]
    pub struct RegisterForm {
        pub username: String,
        pub displayname: String,
        pub password: String,
    }
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    match tera.render("pages/login.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::SeeOther()
        .append_header((actix_web::http::header::LOCATION, "/login"))
        .finish()
}

async fn login_post<A: AuthService>(
    httprequest: HttpRequest,
    form: web::Form<dao::LoginForm>,
    tera: web::Data<Tera>,
    auth_service: web::Data<A>,
) -> impl Responder {
    let credentials = Credentials {
        username: &form.username,
        password: &form.password,
    };
    if auth_service.verify_login(&credentials).await {
        let result = Identity::login(&httprequest.extensions(), credentials.username.to_owned());
        if result.is_ok() {
            return HttpResponse::SeeOther()
                .append_header((actix_web::http::header::LOCATION, "/"))
                .finish();
        }
    }
    let context = Context::new();
    match tera.render("pages/login.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/create_account")]
async fn create_account_form(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    match tera.render("pages/create_account.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_account<A: AuthService>(
    request: HttpRequest,
    form: web::Form<dao::RegisterForm>,
    auth: web::Data<A>,
) -> impl Responder {
    let form_data: dao::RegisterForm = form.into_inner();
    let success = auth
        .register(
            form_data.username.as_str(),
            form_data.password.as_str(),
            form_data.displayname.as_str(),
            Group::Admin,
            form_data.displayname.as_str(),
        )
        .await;
    if success {
        if actix_identity::Identity::login(&request.extensions(), form_data.username).is_ok() {
            return HttpResponse::Created()
                .append_header((actix_web::http::header::LOCATION, "/"))
                .finish();
        }
    }

    HttpResponse::InternalServerError().finish()
}

pub fn configure<T: 'static + AuthService>(
    cfg: &mut web::ServiceConfig,
    auth_service: web::Data<T>,
) {
    cfg.app_data(auth_service)
        .route("/login", web::get().to(login))
        .route("/login", web::post().to(login_post::<T>))
        .route("/logout", web::get().to(logout))
        .service(create_account_form)
        .route("/create_account", web::post().to(create_account::<T>));
}

#[cfg(test)]
mod test {

    use actix_identity::IdentityMiddleware;
    use actix_session::{SessionMiddleware, storage::CookieSessionStore};
    use actix_web::{App, cookie::Key, middleware::Logger, test, web};
    use serde::Serialize;

    use crate::auth::{controller::configure, repositories::Group, service::MockAuthService};
    use mockall::predicate::*;

    #[actix_web::test]
    async fn setup_submit_form_creates_admin_if_not_exist() {
        #[derive(Serialize)]
        pub struct CreateAccountForm<'a> {
            username: &'a str,
            password: &'a str,
            displayname: &'a str,
        }
        let mut mock_auth_service = MockAuthService::default();
        mock_auth_service
            .expect_register()
            .with(
                eq("me"),
                eq("supersecret"),
                eq("Me"),
                eq(Group::Admin),
                eq("Me"),
            )
            .return_const(true);

        let app = test::init_service(
            App::new()
                .wrap(Logger::default())
                .wrap(IdentityMiddleware::default())
                .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    Key::generate(),
                ))
                .configure(|cfg| configure(cfg, web::Data::new(mock_auth_service))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/create_account")
            .set_form(&CreateAccountForm {
                password: "supersecret",
                username: "me",
                displayname: "Me",
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
