mod error;

pub use self::error::{Error, Result};

mod store;
mod mqtt;
pub mod user;
pub mod cluster;

use tokio::sync::broadcast;

use crate::events::Event;
// use mqtt::MqttClient;

#[derive(Clone)]
pub struct ModelChannel {
    pub event_rx: broadcast::Receiver<Event>,
}

/// The ModelManager is responsible for managing the database connection
/// and providing access to the database to the rest of the application.
#[derive(Clone)]
pub struct ModelManager {
    db: store::Database,
    // mqtt: MqttClient,
    channel: ModelChannel,
}

#[allow(dead_code)]
impl ModelManager {

    /// Create a new ModelManager
    pub async fn new(channel: ModelChannel) -> Result<Self> {
        let db = store::new_database_pool().await?;
        Ok(ModelManager { 
            db,
            // mqtt: mqtt_client,
            channel,
        })
    }

    pub async fn run(&mut self) {
        loop {
            if let Ok(event) = self.channel.event_rx.recv().await {
                println!("Received event: {:?}", event);
            };
            
        }
    }

    /// Get a reference to the database
    pub(in crate::model) fn db(&self) -> &store::Database {
        &self.db
    }
}