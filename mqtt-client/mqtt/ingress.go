package mqtt

import (
	"log"
	"encoding/json"
	"database/sql"
	mqtt "github.com/eclipse/paho.mqtt.golang"
	amqp "github.com/rabbitmq/amqp091-go"

)

const (
	exchangeName  = "iot-orchid"
	exchangeType  = "topic"
	queueName     = ""
	routingKey    = "mqtt.register"
	mqttTopic     = "mqtt/register"

)

type RegisterPayload struct {
	DeviceID string `json:"device_id"`
	ClusterID string `json:"cluster_id"`
	UUID string `json:"uuid"`
}

type RegisterResponse struct {
	ControlTopics []string `json:"control_topics"`
	ConfigTopics []string `json:"config_topics"`
	SensorTopics []string `json:"sensor_topics"`
}

// RegistrationHandler struct
type RegistrationHandler struct {
	conn       *amqp.Connection
	channel    *amqp.Channel
	mqttClient mqtt.Client
}

// NewRegistrationHandler creates a new producer instance
func NewRegistrationHandler(amqpURL, mqttBrokerURL string) (*RegistrationHandler, error) {
	conn, err := amqp.Dial(amqpURL)
	if err != nil {
		return nil, err
	}

	ch, err := conn.Channel()
	if err != nil {
		conn.Close()
		return nil, err
	}

	opts := mqtt.NewClientOptions().AddBroker(mqttBrokerURL)
	opts.SetClientID("producer-client")
	mqttClient := mqtt.NewClient(opts)

	producer := &RegistrationHandler{conn: conn, channel: ch, mqttClient: mqttClient}
	return producer, nil
}

// ConnectMQTT connects to the MQTT broker
func (rh *RegistrationHandler) connectMQTT() error {
	token := rh.mqttClient.Connect()
	if token.Wait() && token.Error() != nil {
		return token.Error()
	}
	return nil
}

// Close closes the producer connection and channel
func (rh *RegistrationHandler) Close() {
	if rh.channel != nil {
		rh.channel.Close()
	}
	if rh.conn != nil {
		rh.conn.Close()
	}
	if rh.mqttClient.IsConnected() {
		rh.mqttClient.Disconnect(250)
	}
}

// PublishMessage publishes a message to the specified exchange with a routing key
//allow unused
func (rh *RegistrationHandler) publishMessage(exchange, routingKey, message string) error {
	err := rh.channel.ExchangeDeclare(
		exchange,
		exchangeType,
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		return err
	}

	err = rh.channel.Publish(
		exchange,
		routingKey,
		false,
		false,
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        []byte(message),
		},
	)
	if err != nil {
		return err
	}
	log.Printf("Published message: %s", message)
	return nil
}

// HandleMQTTMessage handles incoming MQTT messages
func (rh *RegistrationHandler) handleMQTTMessage(client mqtt.Client, msg mqtt.Message) {
	log.Printf("Received MQTT message on topic %s: %s", msg.Topic(), msg.Payload())

	msgPayload := msg.Payload()
	registerPayload := RegisterPayload{}
	err := json.Unmarshal(msgPayload, &registerPayload)

	if err != nil {
		log.Printf("Failed to unmarshal message payload: %v", err)
		return
	}

	// Place holder, will be replaced with the actual logic
	responseTopic := msg.Topic() + "/response" + "/" + registerPayload.DeviceID

	registerResponse := RegisterResponse{
		ControlTopics: []string{"control_topic1", "control_topic2"},
		ConfigTopics: []string{"config_topic1", "config_topic2"},
		SensorTopics: []string{"sensor_topic1", "sensor_topic2"},
	}
	response, err := json.Marshal(registerResponse)

	if err != nil {
		log.Printf("Failed to marshal response: %v", err)
		return
	}

	token := rh.mqttClient.Publish(responseTopic, 0, false, response)

	if token.Wait() && token.Error() != nil {
		log.Printf("Failed to publish response to MQTT: %v", token.Error())
	}

	// err := rh.publishMessage(exchangeName, routingKey, string(msg.Payload()))
	// if err != nil {
	// 	log.Printf("Failed to publish message to AMQP: %v", err)
	// }
}

// Subscribe subscribes to the MQTT topic
func (rh *RegistrationHandler) subscribe(topic string) error {
	token := rh.mqttClient.Subscribe(topic, 0, rh.handleMQTTMessage)
	if token.Wait() && token.Error() != nil {
		return token.Error()
	}
	return nil
}

func (rh *RegistrationHandler) Run() {
	err := rh.connectMQTT()
	if err != nil {
		log.Fatalf("Failed to connect to MQTT broker: %v", err)
	}
	defer rh.Close()

	err = rh.subscribe(mqttTopic)
	if err != nil {
		log.Fatalf("Failed to subscribe to MQTT topic: %v", err)
	}

	select {}
}