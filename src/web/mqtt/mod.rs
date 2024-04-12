mod error;
pub use error::{Error, Result};

use paho_mqtt as mqtt;

use crate::events::{Event, EventType, MqttEvent};
use mqtt::{AsyncClient, AsyncReceiver, ConnectOptionsBuilder, Message};
use tokio::sync::mpsc::Sender;

pub struct MqttIngressManager {
    from_broker: AsyncReceiver<Option<Message>>,
    to_event: Sender<Event>,
}

impl MqttIngressManager {
    pub async fn new(client: &mut AsyncClient, to_event: Sender<Event>) -> Result<Self> {
        let stream = client.get_stream(100);

        client
            .connect(ConnectOptionsBuilder::new().clean_session(true).finalize())
            .await?;

        client
            .subscribe("iot-orchid/registration/request", mqtt::QOS_0)
            .await?;
            

        Ok(Self {
            from_broker: stream,
            to_event,
        })
    }

    pub async fn run(&mut self) {
        loop {
            let Ok(message) = self.from_broker.recv().await else {
                continue;
            };

            match message {
                Some(message) => {
                    // let payload = message.payload_str().unwrap();
                    let topic = message.topic().to_string();

                    let event = Event {
                        event_type: EventType::Mqtt,
                        mqtt_event: Some(MqttEvent {
                            action: "message".to_string(),
                            topic,
                            // payload,
                        }),
                    };

                    println!("Received MQTT message: {:?}", event);
                    self.to_event.send(event).await.unwrap();
                }

                None => {
                    continue;
                }
            }
        }
    }
}

pub async fn create_mqtt_client() -> paho_mqtt::AsyncClient {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
        .client_id("iot-orchid-web")
        .finalize();

    if let Ok(client) = mqtt::AsyncClient::new(create_opts) {
        return client;
    }

    panic!("Failed to create MQTT client");
}
