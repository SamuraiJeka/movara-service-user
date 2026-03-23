use sqlx::PgPool;
use async_trait::async_trait;

use crate::domain::user::User;
use crate::domain::repository::UserRepository;

pub struct PgUserRepository {
    pub pool: PgPool,
}


#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(
        &self,
        email: String,
        password_hash: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result = sqlx::query!(
            "INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash",
            email,
            password_hash,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row, |User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        }))
    }

    async fn get_by_id(
            &self,
            id: i32,
        ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result = sqlx::query!(
            "SELECT id, email, password_hash FROM users WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row, |User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        }))
    }

    async fn get_by_email(
            &self,
            email: String,
        ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result = sqlx::query!(
            "SELECT id, email, password_hash FROM users WHERE email = $1",
            email,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row|User{
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        }))
    }
}