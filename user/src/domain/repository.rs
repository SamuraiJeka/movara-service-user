use async_trait::async_trait;

use crate::domain::user::User;


#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        email: String,
        password_hash: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;

    async fn get_by_id(
        &self,
        id: i32,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;

    async fn get_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;
}
