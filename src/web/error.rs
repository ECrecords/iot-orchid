use crate::auth;
use crate::model;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ModelError(model::Error),
    AuthError(auth::Error),
    LoginFailPwdNotMatch,
    LoginFailUserNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<model::Error> for Error {
    fn from(err: model::Error) -> Self {
        Error::ModelError(err)
    }
}

impl From<auth::Error> for Error {
    fn from(err: auth::Error) -> Self {
        Error::AuthError(err)
    }
}

impl From<Error> for StatusCode {
    fn from(err: Error) -> Self {
        #[allow(unreachable_patterns)]
        match err {
            // TODO: map to correct status code
            Error::ModelError(_) => StatusCode::INTERNAL_SERVER_ERROR,


            Error::AuthError(auth_err) => match auth_err {
                auth::Error::JWTCreationError => StatusCode::INTERNAL_SERVER_ERROR,
                auth::Error::JWTValidationError => StatusCode::UNAUTHORIZED,
                auth::Error::JWTSecrectNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            },

            Error::LoginFailPwdNotMatch | Error::LoginFailUserNotFound => StatusCode::UNAUTHORIZED,
            // fall back to internal server error
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        
        let status: StatusCode = self.into();

        status.into_response()
    }
}


impl std::error::Error for Error {}