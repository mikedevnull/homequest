use actix_web::web;
use sqlx::SqlitePool;

mod controller;
mod service;
mod models;
mod repositories;

pub use service::DefaultAuthService;
pub use service::AuthService;
pub use repositories::Group;

pub fn configure(cfg: &mut web::ServiceConfig, db: SqlitePool) {
    let auth_service = service::DefaultAuthService::new(db);
    controller::configure(cfg, web::Data::new(auth_service));
}