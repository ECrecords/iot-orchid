use tokio::sync::mpsc::{Receiver, Sender};

pub struct EventChannels {
    pub model_rx: Receiver<Event>,
    pub event_model_tx: Sender<Event>,
    
    pub web_rx: Receiver<Event>,
    // pub to_web: Sender<Event>,
}

pub struct EventManager {
    channels: EventChannels,
}

// Define the types of events we can handle. Currently, only MQTT events are considered.
#[derive(Debug, Clone)]
pub enum EventType {
    Mqtt,
}

// Struct to represent MQTT events with action, topic, and payload details.
#[derive(Debug, Clone)]
pub struct MqttEvent {
    pub action: String,
    pub topic: String,
    // pub payload: String,
}

// General event struct that can be extended to include different types of events.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub mqtt_event: Option<MqttEvent>,
}

impl EventManager {
    pub async fn new(channels: EventChannels) -> Self {
        EventManager { channels }
    }

    pub async fn run(&mut self) {
        loop {
            let event = self.channels.web_rx.recv().await;

            match event {
                Some(event) => match event.event_type {
                    EventType::Mqtt => {
                        let mqtt_event = event.mqtt_event.unwrap();
                        self.handle_mqtt_event(mqtt_event).await;
                    }
                },
                None => {
                    continue;
                }
            };
        }
    }

    async fn handle_mqtt_event(&self, event: MqttEvent) {
        println!("Handling MQTT event: {:?}", event);
    }
}
