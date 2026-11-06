use std::net::TcpListener;

use crate::{ routes};
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key, dev::Server, middleware::Logger, web};
use sqlx::SqlitePool;
use tera::Tera;

pub fn run(
    listener: TcpListener,
    db: SqlitePool,
    secret_key: Option<Key>,
) -> Result<Server, std::io::Error> {
    

    let secret_key = secret_key.unwrap_or_else(|| Key::generate());

    let mut tera = Tera::default();
    tera.load_from_glob("templates/**/*.html")
        .expect("Failed to load templates");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::ThinData(db.clone()))
            .configure(|cfg| routes::configure(cfg, db.clone()))
    })
    .workers(2)
    .listen(listener)?
    .run();
    Ok(server)
}
