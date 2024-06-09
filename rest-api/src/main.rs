mod auth;
mod context;
mod model;
mod web;

use tokio;
use amqprs::{
    callbacks,
    security::SecurityCredentials,
    connection::{OpenConnectionArguments, Connection},
};

#[macro_use]
extern crate log;

async fn amqp_init() -> Connection {
    let args = OpenConnectionArguments::new("localhost", 5672, "guest", "guest");

    let conn = Connection::open(&args).await.unwrap();

    match conn.register_callback(callbacks::DefaultConnectionCallback).await {
        Ok(_) => (),
        Err(e) => error!("Failed to register channel callback: {}", e),
    };

    return conn;
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

    let conn = amqp_init().await;
    let addr = format!("{}:{}", address, port);

    let app = match web::initalize_app(&conn).await {
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
