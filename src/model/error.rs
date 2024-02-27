
use serde::Serialize;
use crate::model::store;

// Error handling for the store
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    Store(store::Error),
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

impl std::error::Error for Error{}