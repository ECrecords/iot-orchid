package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"strings"
	"time"

	MQTT "github.com/eclipse/paho.mqtt.golang"
)

const (
	brokerHost      = "localhost"
	port            = 1883
	registerTopic   = "register/request/#"
	responseTopic  = "register/response/%s"
	registrationAck = "pod/request/%s+POD_UUID"
)

type MQTTDeviceRegistrar struct {
	client MQTT.Client
}

func NewMQTTDeviceRegistrar(brokerHost string, port int) *MQTTDeviceRegistrar {
	opts := MQTT.NewClientOptions().AddBroker(fmt.Sprintf("tcp://%s:%d", brokerHost, port))
	client := MQTT.NewClient(opts)
	return &MQTTDeviceRegistrar{client: client}
}

func (d *MQTTDeviceRegistrar) onMessage(client MQTT.Client, msg MQTT.Message) {
	topic := msg.Topic()
	payload := string(msg.Payload())

	fmt.Printf("Received message on topic %s: %s\n", topic, payload)

	parts := strings.Split(topic, "/")
	if len(parts) < 3 {
		log.Printf("Invalid topic format: %s\n", topic)
		return
	}

	uuid := parts[2]
	fmt.Printf("UUID: %s\n", uuid)

	// Add your logic here:
	// - Find the device in the database
	// - Determine which pod image to spin up for the device
	// - Handle device groups

	// Respond with the topic in which the spun-up pod is listening on
	responseTopic := fmt.Sprintf(responseTopic, uuid)
	ackTopic := fmt.Sprintf(registrationAck, uuid)
	d.publishMessage(responseTopic, ackTopic)
}

func (d *MQTTDeviceRegistrar) publishMessage(topic string, message string) {
	token := d.client.Publish(topic, 0, false, message)
	token.Wait()
	if token.Error() != nil {
		fmt.Printf("Error publishing message: %v\n", token.Error())
	}
}

func (d *MQTTDeviceRegistrar) ConnectSubscribe() {
	if token := d.client.Connect(); token.Wait() && token.Error() != nil {
		log.Fatalf("Error connecting to MQTT broker: %v\n", token.Error())
	}

	if token := d.client.Subscribe(registerTopic, 0, d.onMessage); token.Wait() && token.Error() != nil {
		log.Fatalf("Error subscribing to topic: %v\n", token.Error())
	}
}

func (d *MQTTDeviceRegistrar) StartLoop() {
	go func() {
		for {
			time.Sleep(5 * time.Second) // Simulate device operations
		}
	}()
}

func main() {
	mqttRegistrar := NewMQTTDeviceRegistrar(brokerHost, port)
	mqttRegistrar.ConnectSubscribe()
	mqttRegistrar.StartLoop()

	// Wait for a termination signal (e.g., Ctrl+C) to gracefully disconnect
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	<-c
	fmt.Println("Disconnecting from MQTT broker...")
	mqttRegistrar.client.Disconnect(250)
	fmt.Println("Disconnected.")
}
