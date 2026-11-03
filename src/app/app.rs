use std::sync::Arc;

use axum::Router;
use axum_login::{AuthManagerLayerBuilder, login_required, tower_sessions};
use tokio::net::TcpListener;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};

use super::config;
use crate::{
    app::state::AppState,
    auth::{self, Backend},
    db,
};

pub async fn run() {
    let config = config::load_configuration().unwrap();
    let db_pool = db::Database::new(config.clone().into()).await.unwrap();

    let shared_state = Arc::new(AppState { config, db_pool });
    let router = Router::new().with_state(Arc::clone(&shared_state));
    //.layer(middleware::from_fn(logging_middleware));

    // Build the listener.
    let addr = format!("0.0.0.0:{}", shared_state.config.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);

    // Generate a cryptographic key to sign the session cookie.

    let session_layer = SessionManagerLayer::new(tower_sessions::MemoryStore::default())
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = auth::Backend { db: db_pool.pool };
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = router.route_layer(login_required!(Backend, login_url = "/login"));

    // Start the API service.
    axum::serve(listener, router).await.unwrap();

    tracing::info!("server shutdown successfully.");
}
