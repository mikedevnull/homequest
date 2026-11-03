use crate::{config, db, routes};
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key, middleware::Logger, web};
use tera::Tera;

pub async fn run() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let config = config::load_configuration().unwrap();
    let db = db::Database::new(config.clone().into()).await.unwrap();
    db.migrate()
        .await
        .expect("Failed to apply database migrations");

    let secret_key = Key::generate();
    let mut tera = Tera::default();
    tera.load_from_glob("templates/**/*.html")
        .expect("Failed to load templates");

    let bind_addr = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::ThinData(db.clone()))
            .configure(routes::configure)
    })
    .workers(2)
    .bind(bind_addr)?
    .run()
    .await
}
