use serde::Serialize;
use paho_mqtt as mqtt;

// Error handling for the store
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    MqttClientCreationFailed(mqtt::Error),

}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<mqtt::Error> for Error {
    fn from(err: mqtt::Error) -> Self {
        Error::MqttClientCreationFailed(err)
    }
    
}

impl std::error::Error for Error{}