use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use tracing_subscriber::EnvFilter;

use crate::application::user_service::UserService;
use crate::config::Config;
use crate::infrastructure::db::create_pool;
use crate::infrastructure::password_hasher::Argon2PasswordHasher;
use crate::infrastructure::repositories::user_repository::PgUserRepository;
use crate::transport::http::handlers;

mod application;
mod config;
mod domain;
mod infrastructure;
mod transport;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env().unwrap_or_else(|e| {
        eprintln!("configuration error: {e}");
        std::process::exit(1);
    });

    let addr = format!("0.0.0.0:{}", config.service_port);
    tracing::info!(%addr, "starting user service");

    let pool = create_pool(&config.database_url)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(error = %e, "database connection failed");
            std::process::exit(1);
        });

    let repo = PgUserRepository { pool };
    let hasher = Arc::new(Argon2PasswordHasher);
    let service = UserService::new(Arc::new(repo), hasher);

    let app = Router::new()
        .route("/users/{user_id}", get(handlers::get_user_by_id))
        .route("/users", get(handlers::get_user_by_email).post(handlers::register))
        .with_state(service);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(error = %e, "failed to bind listener");
            std::process::exit(1);
        });

    tracing::info!("listening");
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!(error = %e, "server error");
        std::process::exit(1);
    }
}
