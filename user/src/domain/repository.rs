use async_trait::async_trait;

use crate::domain::error::DomainError;
use crate::domain::user::User;


#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        email: String,
        password_hash: String
    ) -> Result<User, DomainError>;

    async fn get_by_id(
        &self,
        id: i32
    ) -> Result<Option<User>, DomainError>;

    async fn get_by_email(
        &self,
        email: String
    ) -> Result<Option<User>, DomainError>;
}
