#[macro_use]
extern crate rocket;

use paho_mqtt as mqtt;
use rocket::tokio::sync;
use std::{process, time};
use std::sync::Arc;

type SharedMQTTClient = Arc<sync::Mutex<mqtt::AsyncClient>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/info/<uid>")]
fn device_details(uid: &str) -> String {
    let test = format!("{}", uid);
    println!("{}", test);

    uid.to_string()
}

#[post("/subscribe/<topic>")]
async fn topic_subscribe(topic: &str, mqtt_client: &rocket::State<SharedMQTTClient>) -> String {
    let cli = mqtt_client.lock().await;

    match cli.subscribe(topic, 0).await {
        Ok(_) => "Subscribed Succesfully".to_string(),
        Err(_) => "Failed to Subscribed".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
        .client_id("Mosquitto Broker")
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts).expect("Error creating MQTT client.");

    let broker_connect_config = mqtt::ConnectOptionsBuilder::new()
        .connect_timeout(time::Duration::new(10, 0))
        .finalize();

    let tok = cli.connect(broker_connect_config);

    match tok.wait() {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Failed to connect to MQTT broker because {}", error);
            process::exit(-1);
        }
    };

    let mqtt_client = Arc::new(sync::Mutex::new(cli));

    rocket::build()
        .mount("/", routes![index, topic_subscribe])
        .manage(mqtt_client)
}
