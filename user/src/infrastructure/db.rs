use sqlx::PgPool;
use std::env;


pub async fn create_pool() -> PgPool {
    let database = env::var("DATABASE_URL")
    .expect("DATABASE_URL not exist");
    PgPool::connect(&database)
    .await
    .expect("connection failed")
}
