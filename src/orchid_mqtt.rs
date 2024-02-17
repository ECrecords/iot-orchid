use std::str::FromStr;

use paho_mqtt::message;
use paho_mqtt::AsyncClient;
use paho_mqtt::Message;
use uuid::Uuid;

/// A cluster of devices
#[derive(Clone)]
#[allow(unused)]
pub struct Cluster {
    uuid: Uuid,
    location: String,
    devices: Vec<std::rc::Rc<Device>>,
}

/// A device in the cluster of devices
#[derive(Clone)]
#[allow(unused)]
pub struct Device {
    token: String,
    cluster: std::rc::Rc<Cluster>,
}

impl Device {
    pub fn topic(&self) -> impl Into<String> {
        format!(
            "cluster/{}/{}/device/{}",
            self.cluster.location, self.cluster.uuid.as_hyphenated(), self.token
        )
    }
}

impl Cluster {
    pub fn new<S>(location: S) -> Self
    where S: Into<String>
    {
        Cluster {
            uuid: Uuid::new_v4(),
            location: location.into(),
            devices: vec![],
        }
    }

    pub fn new_device<S>(&self, token: S) -> Result<String, String>
    where S: Into<String>
    {   

        

        let device = Device {
            token: token.into(),
            cluster: std::rc::Rc::new(self.clone()),
        };

        Ok(device.topic().into())
    }

    pub fn location<S>(&self, location: S) -> Self
    where S: Into<String>
    {
        self.location(location.into())
    }

    pub fn topic(&self) -> impl Into<String> {
        format!("cluster/{}/{}", self.location, self.uuid.as_hyphenated())
    }

}

// enum for topics
pub enum Topic {
    Ping,
    Pong,
    Device,
    Server,
    Register,
    Response,
    Error,
    Unknown,
}

trait IntoTopic {
    fn IntoTopic(&self) -> String;
}

impl Topic {
    pub fn to_string(&self) -> String {
        match self {
            Topic::Ping => "ping".to_string(),
            Topic::Pong => "pong".to_string(),
            Topic::Device => "device".to_string(),
            Topic::Server => "server".to_string(),
            Topic::Register => "register".to_string(),
            Topic::Response => "response".to_string(),
            Topic::Error => "error".to_string(),
            Topic::Unknown => "unknown".to_string(),
        }
    }

    pub fn all() -> Vec<Topic> {
        vec![
            Topic::Ping,
            Topic::Pong,
            Topic::Device,
            Topic::Server,
            Topic::Register,
            Topic::Response,
            Topic::Error,
            Topic::Unknown,
        ]
    }
}

impl FromStr for Topic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ping" => Ok(Topic::Ping),
            "pong" => Ok(Topic::Pong),
            "device" => Ok(Topic::Device),
            "server" => Ok(Topic::Server),
            "register" => Ok(Topic::Register),
            "response" => Ok(Topic::Response),
            "error" => Ok(Topic::Error),
            _ => Ok(Topic::Unknown),
        }
    }
}

pub fn create_closure() -> impl FnMut(&AsyncClient, Option<Message>) + Send + 'static {
    move |client: &AsyncClient, message: Option<Message>| {
        if message.is_none() {
            return;
        }

        let message = message.unwrap();

        // match topic
        let topic = message.topic().split("/").collect::<Vec<&str>>();
        let topic = Topic::from_str(topic[0]);

        match topic {
            Ok(Topic::Register) => {}

            Err(_) => {
                println!("Error parsing topic");
            }

            _ => {
                println!("Unknown topic");
            }
        }
    }
}
