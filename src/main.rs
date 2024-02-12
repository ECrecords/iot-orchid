#[macro_use]
extern crate rocket;

mod ping;


use paho_mqtt as mqtt;
use std::collections::HashSet;
use std::sync::Arc;
use std::process;
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

/// Add a device to a cluster
/// 
/// # Arguments
/// 
/// * `cluster_id` - The unique identifier of the cluster
/// * `device_id` - The unique identifier of the device
/// * `mqtt_client` - The shared MQTT client
/// * `registered` - The shared set of registered topics
/// 
#[post("/clusters/<cluster_id>/devices/<device_id>")]
async fn add_device_to_cluster(
    cluster_id: &str,
    device_id: &str,
    mqtt_client: &rocket::State<SharedMQTTClient>,
) {
    let cli = mqtt_client.lock().await;
    let mut reg_lock = registered.lock().await;

    let topic = format!("cluster/{}/device/{}", cluster_id, device_id);
    if let Ok(_) = cli.subscribe(&topic, mqtt::QOS_1).await {
        reg_lock.insert(topic);
    }
}


mod orchid_mqtt;

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

    cli.set_message_callback(orchid_mqtt::create_closure());
    // register to all topics in orchid_mqtt Topic enum
    for topic in orchid_mqtt::Topic::all() {
        let topic = format!("{}/#", topic.to_string());
        println!("Subscribing to {}", topic);
        cli.subscribe(topic, mqtt::QOS_1);
    }

    cli.subscribe("cluster/us-1/+/device/+", mqtt::QOS_0);

    let mqtt_client = Arc::new(sync::Mutex::new(cli));

    let registered: Arc<sync::Mutex<HashSet<String>>> =
        Arc::new(sync::Mutex::new(HashSet::new()));

    rocket::build()
        .mount("/api/v1/", routes![index,  device_details])
        .manage(mqtt_client)
    
}
