mod model;
mod web;
mod auth;
mod context;
mod events;
use tokio;

#[macro_use]
extern crate log;

// used to create the event queue between the event and model layers
use tokio::sync::mpsc;

use events::{Event, EventManager, EventChannels};
use model::ModelChannel;

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

    // create the event queues
    let (to_event_from_model, from_model) = mpsc::channel(100);
    let (to_model, from_event_to_model) = mpsc::channel(100);

    let (to_event_from_web, from_web) = mpsc::channel(100);
    let (to_web, from_event_to_web) = mpsc::channel(100);

    let event_channels = EventChannels {
        to_web,
        from_web,
        to_model,
        from_model,
    };

    let event_manager = EventManager::new(event_channels).await;

    let model_channels = ModelChannel {
        to_event: to_event_from_model,
        from_event: from_event_to_model,
    };

    let app = match web::get_routes(model_channels).await {
        Ok(app) => app,
        Err(e) => panic!("Failed to create routes: {}", e),
    };

    let listener = tokio::net::TcpListener::bind(&addr).await
        .expect("Failed to bind to address");

    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => println!("Server error: {}", e),
    }
}