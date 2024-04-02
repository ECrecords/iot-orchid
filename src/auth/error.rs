use std::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    JWTCreationError,
    JWTValidationError,
    JWTSecrectNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::JWTCreationError => write!(f, "Failed to create JWT"),
            Error::JWTValidationError => write!(f, "Failed to validate JWT"),
            Error::JWTSecrectNotFound => write!(f, "JWT secret not found"),
        }
    }
}

impl error::Error for Error {}