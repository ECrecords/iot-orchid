use serde::Serialize;
use crate::model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    ModelError(model::Error),
    JWTCreationError,
    JWTValidationError,
    
}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// into
impl From<model::Error> for Error {
    fn from(err: model::Error) -> Self {
        Error::ModelError(err)
    }
    
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::JWTCreationError
    }
    
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => Error::JWTValidationError,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::JWTValidationError,
            _ => Error::JWTCreationError,
        }
    }
    
}

impl From<Error> for axum::http::StatusCode {
    fn from(err: Error) -> Self {
        match err {
            Error::ModelError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::JWTCreationError => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::JWTValidationError => axum::http::StatusCode::UNAUTHORIZED,
        }
    }
}

impl std::error::Error for Error{}