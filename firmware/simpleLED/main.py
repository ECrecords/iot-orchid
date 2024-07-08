from machine import Pin, reset
import network
import utime
from umqtt.simple import MQTTClient
import ujson

DEVICE_ID = "3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e"
MQTT_BROKER = "192.168.0.181"
SSID = "Apartment 306"
PASSWORD = "09214354"
MAX_WIFI_ATTEMPTS = 10
MAX_MQTT_ATTEMPTS = 5
TEMP_SENSE_PIN = 4

REGISTER_RESPONSE_TOPIC = f"mqtt/register/{DEVICE_ID}"

CONTROL_TOPICS = ["led"]
STATUS_TOPICS = ["led", "temperature"]
CONFIG_TOPICS = []

REGISTRATION_MSG = b'{"device_id":"3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e","controlTopics":["led"], "statusTopics":["led", "temperature"], "configTopics":[]}'

led = Pin("WL_GPIO0", Pin.OUT)
sensor = machine.ADC(TEMP_SENSE_PIN)
cluster_id = None

def read_temperature():
    adc_value = sensor.read_u16()
    volt = (3.3/65535) * adc_value
    temperature = 27 - (volt - 0.706)/0.001721
    return round(temperature, 1)

isRegistered = False

def message_handler(topic, msg):
    global cluster_id
    global isRegistered
    print(f"Received message on topic {topic}: {msg}")
    if topic == REGISTER_RESPONSE_TOPIC.encode():
        if isRegistered:
            print("Device Already Registered")
            return
        else:
            isRegistered = True
                
        try:
            message = ujson.loads(msg)
            cluster_id = message.get("cluster_id")
            if cluster_id:
                # Subscribe to control topics
                for control_topic in CONTROL_TOPICS:
                    client.subscribe(f"{cluster_id}/{DEVICE_ID}/control/{control_topic}")
                # Subscribe to status topics
                for status_topic in STATUS_TOPICS:
                    client.subscribe(f"{cluster_id}/{DEVICE_ID}/status/{status_topic}")
                print(f"Subscribed to topics: control and status topics for cluster {cluster_id}")
        except ValueError:
            print("Failed to parse JSON message")      
    else:
        if cluster_id and topic.startswith(f"{cluster_id}/{DEVICE_ID}/control/"):
            try:
                message = ujson.loads(msg)
                control = message.get("control")
                topic_suffix = topic.decode().split('/')[-1]
                if topic_suffix == "led":
                    if control == "on":
                        led.on()
                    elif control == "off":
                        led.off()
            except ValueError:
                print("Failed to parse JSON message")
        elif cluster_id and topic.startswith(f"{cluster_id}/{DEVICE_ID}/status/"):
            topic_suffix = topic.decode().split('/')[-1]
            if topic_suffix == "temperature":
                temperature = read_temperature()
                client.publish(f"{cluster_id}/{DEVICE_ID}/status/temperature", ujson.dumps({"temperature": temperature}))

def connect_to_wifi():
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    wlan.connect(SSID, PASSWORD)
    
    attempts = 0
    while not wlan.isconnected():
        print("Connecting to network...")
        utime.sleep(1)
        attempts += 1
        if attempts >= MAX_WIFI_ATTEMPTS:
            print("Failed to connect to WiFi after several attempts. Resetting...")
            reset()
    
    print("Connected to WiFi")
    print("Network config:", wlan.ifconfig())

def connect_to_mqtt(client):
    attempts = 0
    while True:
        try:
            client.connect()
            break
        except Exception as e:
            print(f"Failed to connect to MQTT broker: {e}")
            attempts += 1
            if attempts >= MAX_MQTT_ATTEMPTS:
                print("Failed to connect to MQTT broker after several attempts. Resetting...")
                reset()
            utime.sleep(1)

def main():
    connect_to_wifi()
    
    global client
    client = MQTTClient("test_client", MQTT_BROKER, 1883)
    client.set_callback(message_handler)
    
    connect_to_mqtt(client)
    
    client.subscribe(REGISTER_RESPONSE_TOPIC)
    
    client.publish("mqtt/register", REGISTRATION_MSG)
    print("Sent registration message")
    
    while True:
        client.check_msg()

if __name__ == '__main__':
    main()
