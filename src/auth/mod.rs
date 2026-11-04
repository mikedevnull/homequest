use actix_web::web;

use crate::db::Database;


mod controller;
mod service;
mod models;
mod repositories;

pub use service::DefaultAuthService;
pub use service::AuthService;

pub fn configure(cfg: &mut web::ServiceConfig, db: Database) {
    let auth_service = service::DefaultAuthService::new(db);
    controller::configure(cfg, web::Data::new(auth_service));
}