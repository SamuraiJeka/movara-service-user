use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};


pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    return argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
}
