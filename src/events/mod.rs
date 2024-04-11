// Import necessary crates and modules.
mod mqtt;

use tokio::sync::mpsc::{Receiver, Sender};

pub struct EventChannels {
    pub to_web: Sender<Event>,
    pub from_web: Receiver<Event>,
    pub to_model: Sender<Event>,
    pub from_model: Receiver<Event>,
}

pub struct EventManager {
    channels: EventChannels,
}

// Define the types of events we can handle. Currently, only MQTT events are considered.
#[derive(Debug, Clone)]
enum EventType {
    Mqtt,
}

// Struct to represent MQTT events with action, topic, and payload details.
#[derive(Debug, Clone)]
struct MqttEvent {
    action: String,
    topic: String,
    payload: String,
}

// General event struct that can be extended to include different types of events.
#[derive(Debug, Clone)]
pub struct Event {
    event_type: EventType,
    mqtt_event: Option<MqttEvent>,
}

impl EventManager {
    pub async fn new(channels: EventChannels) -> Self {
        EventManager {
            channels,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let event = self.channels.from_web.recv().await.unwrap();

            let future = match event.event_type {
                EventType::Mqtt => {
                    let mqtt_event = event.mqtt_event.unwrap();
                    self.handle_mqtt_event(mqtt_event) // Await the future here.
                }
            };

            tokio::spawn(future);
        }
    }

    async fn handle_mqtt_event(&mut self, event: MqttEvent) {
        todo!()
    }
    
}