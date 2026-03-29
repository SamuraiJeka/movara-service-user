pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, ()>;
}
