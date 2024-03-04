use serde::Serialize;
use crate::model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    ModelError(model::Error),
}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error{}

// into
impl From<model::Error> for Error {
    fn from(err: model::Error) -> Self {
        Error::ModelError(err)
    }
    
}