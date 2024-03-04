use std::error;

use crate::model::store;
use serde::Serialize;

// Error handling for the store
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    Store(store::Error),
    DatabaseError,
    EntityNotFound,
    UserNotFound,
}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<store::Error> for Error {
    fn from(err: store::Error) -> Self {
        Error::Store(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::EntityNotFound,
            _ => Error::DatabaseError,
        }
    }
}

impl std::error::Error for Error {}
