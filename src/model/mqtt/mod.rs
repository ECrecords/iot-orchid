mod error;

#[allow(unused_imports)]
pub use self::error::{Error, Result};

use paho_mqtt as mqtt;

pub type MqttClient = mqtt::AsyncClient;

pub async fn new_mqtt_client() -> Result<MqttClient> {
    todo!();
}