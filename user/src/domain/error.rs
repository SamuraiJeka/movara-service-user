use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("email already registered")]
    DuplicateEmail,
    #[error("internal error")]
    Internal,
}
