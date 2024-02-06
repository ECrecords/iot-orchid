#[macro_use]
extern crate rocket;



use paho_mqtt as mqtt;
use std::collections::HashSet;
use std::sync::Arc;
use std::process;

#[allow(unused_imports)]
use rusqlite::{Connection, Result};

use rocket::tokio::{
    sync,
    time
};

#[allow(unused_imports)]
use rocket::response;

#[allow(unused_imports)]
use rocket::http;


type SharedMQTTClient = Arc<sync::Mutex<mqtt::AsyncClient>>;
type SharedRegisteredTopics = Arc<sync::Mutex<HashSet<String>>>;
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

#[post("/register/<device_token>")]
async fn register_device(
    device_token: &str,
    mqtt_client: &rocket::State<SharedMQTTClient>,
    registered: &rocket::State<SharedRegisteredTopics>,
) {
    let cli = mqtt_client.lock().await;
    let mut reg_lock = registered.lock().await;

    // is device is marked as registered?
    if reg_lock.get(device_token).is_some() {
        
        let ( tx, mut rx) = sync::mpsc::channel(1);
        // TODO REPLACE WITH ACTUAL SERVER ID
        let response_topic = format!("ping/reponse/{}/{}", device_token, "SERVER_ID");
        let ping_topic = format!("ping/{}/{}", "SERVER_ID", device_token);
        
        // TODO add a timeout to the subscribe await
        if let Ok(_) = cli.subscribe(&response_topic, mqtt::QOS_1).await {
            
            cli.set_message_callback(move |_, msg| {
                if let Some(message) = msg {
                    if message.topic() == response_topic {
                        let _ = tx.send(message);
                    }
                }
            });

            let msg = mqtt::MessageBuilder::new()
                .topic(ping_topic)
                .finalize();

            cli.publish(msg);

            match time::timeout(time::Duration::from_secs(10), rx.recv()).await {
                Ok(_) => {
                    
                },
                Err(_) => {}
            }



        }
    }

}

#[launch]
fn rocket() -> _ {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://192.168.0.159:1883")
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

    let registered: Arc<sync::Mutex<HashSet<String>>> =
        Arc::new(sync::Mutex::new(HashSet::new()));

    rocket::build()
        .mount("/", routes![index, register_device])
        .manage(mqtt_client)
        .manage(registered)
}
