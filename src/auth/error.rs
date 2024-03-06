use std::error;
use axum::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    JWTCreationError,
    JWTValidationError,
    JWTSecrectNotFound,
}

impl From<Error> for StatusCode {
    fn from(err: Error) -> Self {
        match err {
            Error::JWTCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::JWTValidationError => StatusCode::UNAUTHORIZED,
            Error::JWTSecrectNotFound => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
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