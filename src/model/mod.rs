mod error;

pub use self::error::{Error, Result};

mod store;
pub mod user;
pub mod cluster;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::events::Event;
pub struct ModelChannel {
    pub to_event: Sender<Event>,
    pub from_event: Receiver<Event>,
}

/// The ModelManager is responsible for managing the database connection
/// and providing access to the database to the rest of the application.
#[derive(Clone)]
pub struct ModelManager {
    db: store::Database
}

#[allow(dead_code)]
impl ModelManager {

    /// Create a new ModelManager
    pub async fn new(channels: ModelChannel) -> Result<Self> {
        let db = store::new_database_pool().await?;
        Ok(ModelManager { db })
    }

    /// Get a reference to the database
    pub(in crate::model) fn db(&self) -> &store::Database {
        &self.db
    }
}