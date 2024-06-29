mod error;
use amqprs::connection::Connection;

pub use self::error::{Error, Result};
mod store;
pub mod user;
pub mod cluster;

/// The ModelManager is responsible for managing the database connection
/// and providing access to the database to the rest of the application.
#[derive(Clone)]
pub struct ModelManager {
    db: store::Database,
    amqp: Connection,
}

#[allow(dead_code)]
impl ModelManager {

    /// Create a new ModelManager
    pub async fn new(conn: &Connection) -> Result<Self> {
        let db = store::new_database_pool().await?;

        // Check if the AMQP connection is open
        if !conn.is_open() {
            return Err(Error::AMQPError("AMQP connection is not open".to_string()));
        }

        Ok(ModelManager { 
            db,
            amqp: conn.clone(),
        })
    }

    /// Get a reference to the database
    pub(in crate::model) fn db(&self) -> &store::Database {
        &self.db
    }

    /// Get a reference to the AMQP connection
    pub(in crate::model) fn amqp(&self) -> &Connection {
        &self.amqp
    }
}