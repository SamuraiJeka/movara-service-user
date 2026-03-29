use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("could not hash password")]
    PasswordHash,
}
