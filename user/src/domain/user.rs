#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[allow(dead_code)]
    pub password_hash: String,
}
