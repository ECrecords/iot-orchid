mod error;

use std::time::Duration;

#[allow(unused_imports)]
pub use self::error::{Error, Result};

use mqtt::{AsyncClient, Properties, ReasonCode};
use mqtt::ConnectOptionsBuilder;
use paho_mqtt as mqtt;

pub type MqttClient = AsyncClient;

pub async fn new_mqtt_client() -> Result<MqttClient> {
    let cli = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
        .client_id("iot-orchid-broker")
        .create_client()?;

    cli.set_disconnected_callback(disconnect_cb);
    cli.set_connected_callback(connect_cb);

    let con_opts = ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(60 * 60 * 24 * 5))
        .automatic_reconnect(Duration::from_secs(10), Duration::from_secs(60 * 10))
        .connect_timeout(Duration::from_secs(30))
        .finalize();

    cli.connect(con_opts).await?;

    Ok(cli)
}


fn disconnect_cb(_cli: &MqttClient, _prop: Properties, reason: ReasonCode) {
    println!("Disconnected: {:?}", reason);
}

fn connect_cb(cli: &MqttClient) {
    println!("Connected to: {:?}", cli.client_id());
}