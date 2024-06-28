package mqtt

import (
	"fmt"
	"log"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	amqp "github.com/rabbitmq/amqp091-go"
)

const (
	amqpExchange = "iotorchid.mqtt"
)

type RegistrationHandler struct {
	amqpConn     *amqp.Connection
	mqttClient   mqtt.Client
	incomingChan *amqp.Channel
}

var registrationTopics = []string{"mqtt/registration"}

func (regHandler *RegistrationHandler) messagePubHandler(client mqtt.Client, msg mqtt.Message) {
	log.Printf("Received MQTT message on topic %s: %s", msg.Topic(), msg.Payload())
	if msg.Topic() == "mqtt/registration" {
		if regHandler.incomingChan != nil {
			log.Println("Publishing message to AMQP")
			err := regHandler.incomingChan.Publish(
				amqpExchange,
				"mqtt.registration",
				false,
				false,
				amqp.Publishing{
					ContentType: "text/plain",
					Body:        msg.Payload(),
				},
			)
			if err != nil {
				log.Printf("Failed to publish message to AMQP: %v", err)
			} else {
				log.Println("Message published to AMQP successfully")
			}
		} else {
			log.Println("Incoming AMQP channel is nil, cannot publish message")
		}
	}
}

func NewRegistrationHandler(amqpURL, mqttBrokerURL string) (*RegistrationHandler, error) {
	regHandler := &RegistrationHandler{}

	amqpConn, err := amqp.Dial(amqpURL)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to AMQP broker: %w", err)
	}

	opts := mqtt.NewClientOptions().AddBroker(mqttBrokerURL)
	opts.SetClientID("iot-orchid.registration_handler")
	opts.SetDefaultPublishHandler(regHandler.messagePubHandler)

	mqttClient := mqtt.NewClient(opts)
	if token := mqttClient.Connect(); token.Wait() && token.Error() != nil {
		return nil, fmt.Errorf("failed to connect to MQTT broker: %w", token.Error())
	}

	regHandler.amqpConn = amqpConn
	regHandler.mqttClient = mqttClient
	regHandler.incomingChan = nil

	return regHandler, nil
}

func (regHandler *RegistrationHandler) Start() error {
	for _, topic := range registrationTopics {
		log.Printf("Subscribing to topic %s", topic)
		if token := regHandler.mqttClient.Subscribe(topic, 0, nil); token.Wait() && token.Error() != nil {
			return fmt.Errorf("failed to subscribe to MQTT topic %s: %w", topic, token.Error())
		}
	}

	ch, err := regHandler.amqpConn.Channel()
	if err != nil {
		return fmt.Errorf("failed to open AMQP channel: %w", err)
	}

	err = ch.ExchangeDeclare(
		amqpExchange,
		"topic",
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		return fmt.Errorf("failed to declare AMQP exchange: %w", err)
	}

	regHandler.incomingChan = ch
	return nil
}

func (regHandler *RegistrationHandler) Close() {
	if regHandler.incomingChan != nil {
		_ = regHandler.incomingChan.Close()
	}
	if regHandler.amqpConn != nil {
		_ = regHandler.amqpConn.Close()
	}
	if regHandler.mqttClient.IsConnected() {
		regHandler.mqttClient.Disconnect(250)
	}
}