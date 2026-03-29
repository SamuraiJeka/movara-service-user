use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHasher as Argon2Hasher};

use crate::application::ports::PasswordHasher;

pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Argon2Hasher::hash_password(&argon2, password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| ())
    }
}
