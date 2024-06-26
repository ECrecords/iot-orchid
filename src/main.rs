mod model;
mod web;
mod auth;
mod context;
mod events;
use tokio;

#[macro_use]
extern crate log;


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

    let app = match web::get_routes().await {
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