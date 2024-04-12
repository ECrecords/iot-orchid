mod auth;
mod context;
mod events;
mod model;
mod web;
use tokio;

use crate::events::{Event, EventChannels, EventManager};
use crate::model::ModelChannel;
use tokio::sync::mpsc::{channel, Sender};
use web::mqtt::create_mqtt_client;
use web::mqtt::MqttIngressManager;

#[macro_use]
extern crate log;

pub struct WebChannels {
    to_event: Sender<Event>,
}

pub fn create_channels() -> (WebChannels, EventChannels, ModelChannel) {
    // create the event queues
    let (web_2_event_sender, web_2_event_receiver) = channel(100);
    let (event_2_model_sender, event_2_model_receiver) = channel(100);
    let (model_2_event_sender, model_2_event_receiver) = channel(100);
    let event_channels = EventChannels {
        model_rx: model_2_event_receiver,
        event_model_tx: event_2_model_sender,
        web_rx: web_2_event_receiver,
        // to_web: to_event_from_web,
    };

    let model_channel = ModelChannel {
        event_tx: model_2_event_sender,
        event_rx: event_2_model_receiver,
    };

    let web_channels = WebChannels {
        to_event: web_2_event_sender,
    };

    (web_channels, event_channels, model_channel)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let Ok(port) = std::env::var("IOT_ORCHID_PORT") else {
        panic!("IOT_ORCHID_PORT must be set.");
    };

    let Ok(address) = std::env::var("IOT_ORCHID_ADDRESS") else {
        panic!("IOT_ORCHID_ADDRESS must be set.");
    };

    let addr = format!("{}:{}", address, port);

    let channels = create_channels();

    let mut client = create_mqtt_client().await;

    let mqtt_manager = MqttIngressManager::new(&mut client, channels.0.to_event).await;

    let mut event_manager = EventManager::new(channels.1).await;

    tokio::spawn(async move {
        println!("Starting MQTT ingress manager");
        if let Ok(mut mqtt_manager) = mqtt_manager {
            mqtt_manager.run().await;
        }
    });

    tokio::spawn(async move {
        println!("Starting event manager");
        event_manager.run().await;
    });

    let app = match web::initalize_app(channels.2).await {
        Ok(app) => app,
        Err(e) => panic!("Failed to create routes: {}", e),
    };

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => println!("Server error: {}", e),
    }
}
