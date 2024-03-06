use serde::Serialize;


// Error handling for the store
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    PoolCreationFailed(String),
    MigrationFailed(String)
}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error{}