use sqlx::PgPool;
use async_trait::async_trait;

use crate::domain::error::DomainError;
use crate::domain::repository::UserRepository;
use crate::domain::user::User;

pub struct PgUserRepository {
    pub pool: PgPool,
}

fn map_sqlx_error(e: sqlx::Error) -> DomainError {
    if let sqlx::Error::Database(ref db) = e {
        if db.code().as_deref() == Some("23505") {
            return DomainError::DuplicateEmail;
        }
    }
    tracing::error!(error = ?e, "database error");
    DomainError::Internal
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(
        &self,
        email: String,
        password_hash: String
    ) -> Result<User, DomainError> {
        let row = sqlx::query!(
            "INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash",
            email,
            password_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        })
    }

    async fn get_by_id(
        &self,
        id: i32
    ) -> Result<Option<User>, DomainError> {
        let result = sqlx::query!(
            "SELECT id, email, password_hash FROM users WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.map(|row| User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        }))
    }

    async fn get_by_email(
        &self,
        email: String
    ) -> Result<Option<User>, DomainError> {
        let result = sqlx::query!(
            "SELECT id, email, password_hash FROM users WHERE email = $1",
            email,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.map(|row| User {
            id: row.id,
            email: row.email,
            password_hash: row.password_hash,
        }))
    }
}
