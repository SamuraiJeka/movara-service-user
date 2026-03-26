use std::sync::Arc;

use crate::domain::repository::UserRepository;
use crate::domain::user::User;
use crate::infrastructure::hash_password::hash_password;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let hash = hash_password(&password);

        return self.repo
            .create_user(email, hash)
            .await;
    }

    pub async fn get_by_id(
        &self,
        user_id: i32,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        return self.repo
            .get_by_id(user_id)
            .await;
    }

    pub async fn get_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        return self.repo
            .get_by_email(email)
            .await;
    }
}
