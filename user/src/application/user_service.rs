use std::sync::Arc;

use crate::application::error::ServiceError;
use crate::application::ports::PasswordHasher;
use crate::domain::repository::UserRepository;
use crate::domain::user::User;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
}

impl UserService {
    pub fn new(
        repo: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>
    ) -> Self {
        Self { repo, hasher }
    }

    pub async fn register(
        &self,
        email: String,
        password: String
    ) -> Result<User, ServiceError> {
        let hash = self
            .hasher
            .hash_password(&password)
            .map_err(|()| ServiceError::PasswordHash)?;

        self.repo.create_user(email, hash).await.map_err(Into::into)
    }

    pub async fn get_by_id(
        &self,
        user_id: i32
    ) -> Result<Option<User>, ServiceError> {
        self.repo.get_by_id(user_id)
        .await
        .map_err(Into::into)
    }

    pub async fn get_by_email(
        &self,
        email: String
    ) -> Result<Option<User>, ServiceError> {
        self.repo.get_by_email(email).await.map_err(Into::into)
    }
}
