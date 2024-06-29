use crate::model::store;

use amqprs;
// use crate::events::mqtt;

// Error handling for the store
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    Store(store::Error),
    AMQPError(String),
    DatabaseError,
    // Mqtt(mqtt::Error),
    EntityNotFound,
    UserNotFound,
}

// implment display
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<store::Error> for Error {
    fn from(err: store::Error) -> Self {
        Error::Store(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::EntityNotFound,
            _ => Error::DatabaseError,
        }
    }
}

impl From<amqprs::Error> for Error {
    fn from(err: amqprs::Error) -> Self {
        Error::AMQPError(err.to_string())
    }   
}

// impl From<mqtt::Error> for Error {
//     fn from(err: mqtt::Error) -> Self {
//         Error::Mqtt(err)
//     }
// }

impl std::error::Error for Error {}
