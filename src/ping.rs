use paho_mqtt as mqtt;
use rocket::tokio;
use tokio::sync;
use tokio::time;

const TIMEOUT_SECONDS: u64 = 30;
const MAX_ATTEMPTS: u32 = 5;
const CHANNEL_BUFFER_LEN: usize = 1;

pub enum PingError {
    Mqtt(mqtt::Error),
    SendError(sync::mpsc::error::SendError<String>),
    Timeout(time::error::Elapsed),
}

impl From<tokio::sync::mpsc::error::SendError<String>> for PingError {
    fn from(err: tokio::sync::mpsc::error::SendError<String>) -> PingError {
        PingError::SendError(err)
    }
}

impl From<tokio::time::error::Elapsed> for PingError {
    fn from(err: tokio::time::error::Elapsed) -> PingError {
        PingError::Timeout(err)
    }
}

impl From<mqtt::Error> for PingError {
    fn from(err: mqtt::Error) -> PingError {
        PingError::Mqtt(err)
    }
}

pub async fn ping<S>(token: S, uri: S) -> Result<Option<mqtt::Message>, PingError>
where
    S: Into<String>,
{
    let token = token.into();
    let uri = uri.into();

    let response_topic: String = format!("ping/response/{}/{}", token, "SERVER_ID");
    let ping_topic: String = format!("ping/{}/{}", token, "SERVER_ID");

    let (tx, mut rx) = sync::mpsc::channel::<mqtt::Message>(CHANNEL_BUFFER_LEN);

    let ping_cli = mqtt::CreateOptionsBuilder::new()
        .server_uri(uri)
        .client_id("ping client")
        .create_client()?;

    ping_cli
        .connect(
            mqtt::ConnectOptionsBuilder::new()
                .connect_timeout(time::Duration::from_secs(TIMEOUT_SECONDS))
                .finalize(),
        )
        .await?;

    ping_cli.subscribe(&response_topic, mqtt::QOS_1).await?;

    ping_cli.set_message_callback(move |_, msg| {
        if let Some(message) = msg {
            if message.topic() == response_topic {
                tx.send(message);
            }
        }
    });

    ping_cli
        .publish(
            mqtt::MessageBuilder::new()
                .topic(ping_topic)
                .qos(mqtt::QOS_1)
                .finalize(),
        )
        .await?;

    Ok(time::timeout(time::Duration::from_secs(TIMEOUT_SECONDS), rx.recv()).await?)
}
use paho_mqtt as mqtt;
use rocket::tokio::sync;
use rocket::tokio::time;

pub const TIMEOUT_SECONDS: u64 = 30;
pub const ATTEMPT_MAX: u64 = 5;

#[allow(unused)]
pub async fn ping<S>(token: S, uri: S) -> Result<String, String>
where
    S: Into<String>,
{
    let ping_cli: mqtt::AsyncClient;
    let timeout_sec: time::Duration;

    ping_cli = match mqtt::AsyncClient::new(
        mqtt::CreateOptionsBuilder::new()
            .client_id("ping client")
            .server_uri(uri)
            .finalize(),
    ) {
        Err(_) => {
            return Err(String::from("failed to create mqtt aync client for ping"));
        }

        Ok(cli) => cli,
    };

    timeout_sec = time::Duration::from_secs(TIMEOUT_SECONDS);

    if let Err(err) = ping_cli
        .connect(
            mqtt::ConnectOptionsBuilder::new()
                .connect_timeout(timeout_sec)
                .finalize(),
        )
        .await
    {
        match err {
            mqtt::Error::Io(_) => {
                return Err(String::from("IO error when connecting to mqtt broker"))
            }
            mqtt::Error::Timeout => {
                return Err(String::from(
                    "Connection timed out when connecting to mqtt broker",
                ))
            }
            _ => return Err(String::from("Unknown error when connecting to mqtt broker")),
        }
    };

    let (tx, rx) = sync::mpsc::channel::<String>(1);

    Ok("device responded".to_string())
}

#[cfg(test)]
mod ping_test {
    #[test]
    fn test() {}
}
