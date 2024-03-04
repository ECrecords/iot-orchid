mod error;

pub use self::error::{Error, Result};

mod store;
pub mod user;

/// The ModelManager is responsible for managing the database connection
/// and providing access to the database to the rest of the application.
#[derive(Clone)]
pub struct ModelManager {
    db: store::Database,
}

impl ModelManager {

    /// Create a new ModelManager
    pub async fn new() -> Result<Self> {
        let db = store::new_database_pool().await?;

        Ok(ModelManager { db })
    }

    /// Get a reference to the database
    pub(in crate::model) fn db(&self) -> &store::Database {
        &self.db
    }
}