use std::net::TcpListener;

use actix_web::cookie::Key;
use homequest::{app, db, load_configuration};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let config = load_configuration().unwrap();
    let pool = sqlx::SqlitePool::connect_with(config.clone().into()).await.unwrap();
    db::migrate(pool.clone()).await.expect("Database migration failed");
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind tcp listener");
    let secret_key = config.secretkey.map(|v| Key::derive_from(v.as_bytes()));
    app::run(listener, pool, secret_key)?.await
}
