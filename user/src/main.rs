use std::{env, sync::Arc};
use axum::{Router, routing::{get, post}};


use crate::infrastructure::{db::create_pool, repositories::user_repository::PgUserRepository};
use crate::application::user_service::UserService;
use crate::transport::http::handlers as user_handlers;

mod infrastructure;
mod application;
mod transport;
mod domain;


#[tokio::main]
async fn main() {
    let service_port = env::var("SERVICE_PORT")
        .expect("SERVICE_PORT not exist");

    let service_address = format!("0.0.0.0:{}", service_port);

    let db = create_pool().await;

    let repo = PgUserRepository{pool: db};

    let service = UserService::new(Arc::new(repo));

    let app = Router::new()
        .route("/register", post(user_handlers::register))
        .route("/get_by_id", get(user_handlers::get_user_by_id))
        .route("/get_by_email", get(user_handlers::get_user_by_email))
        .with_state(service);

     let listener = tokio::net::TcpListener::bind(&service_address)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
