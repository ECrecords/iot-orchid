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
