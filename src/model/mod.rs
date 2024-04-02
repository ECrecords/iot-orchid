mod error;

pub use self::error::{Error, Result};

mod store;
mod mqtt;
pub mod user;
pub mod cluster;

/// The ModelManager is responsible for managing the database connection
/// and providing access to the database to the rest of the application.
#[derive(Clone)]
pub struct ModelManager {
    db: store::Database,
    mqtt_client: mqtt::MqttClient,
}

#[allow(dead_code)]
impl ModelManager {

    /// Create a new ModelManager
    pub async fn new() -> Result<Self> {
        let db = store::new_database_pool().await?;
        let mqtt_client = mqtt::new_mqtt_client().await?;
        Ok(ModelManager { db, mqtt_client })
    }

    /// Get a reference to the database
    pub(in crate::model) fn db(&self) -> &store::Database {
        &self.db
    }

    /// Get a reference to the mqtt client
    pub(in crate::model) fn mqtt(&self) -> &mqtt::MqttClient {
        &self.mqtt_client
    }
}