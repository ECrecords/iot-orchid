// use serde::Serialize;
use paho_mqtt as mqtt;

// Error handling for the store
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    MqttClientConnectionTimeout,
    MqttClientCreationFailed,
    MqttClientFailToConnect,

}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<mqtt::errors::Error> for Error {
    fn from(err: mqtt::Error) -> Self {
        match err {
            mqtt::Error::Timeout => Error::MqttClientConnectionTimeout,
            _ => Error::MqttClientCreationFailed

            // Error::MqttClientCreationFailed(err)
        }
    }
    
}

impl std::error::Error for Error{}