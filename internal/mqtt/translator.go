package mqtt

import (
	"fmt"
	"log/slog"
	mqtt "github.com/eclipse/paho.mqtt.golang"
	amqp "github.com/rabbitmq/amqp091-go"
)

type Translator struct {
	amqpConn         *amqp.Connection
	amqpCh           *amqp.Channel
	amqpQ            amqp.Queue
	mqttClient       mqtt.Client
	mqttMsgProcessor MqttMsgProcessor
}

type MqttMsgProcessor func(mqtt.Message) []byte

func NewTranslator(
	amqpURL string,
	amqpQueueName string,
	mqttBrokerURL string,
	mqttClientID string,
) (*Translator, error) {

	t := &Translator{}

	t.createWorkerQueue(amqpURL, amqpQueueName)

	// Translates messages received via specified MQTT topics to a AMQP worker queue
	t.createMqttClient(mqttBrokerURL, mqttClientID)

	return t, nil
}

// createWorkerQueue creates a worker queue for handling AMQP connections and channels.
// It takes the AMQP URL and queue name as parameters and returns an error if any.
func (t *Translator) createWorkerQueue(amqpUrl, queueName string) error {

	var err error

	t.amqpConn, err = amqp.Dial(amqpUrl)
	if err != nil {
		return err
	}

	t.amqpCh, err = t.amqpConn.Channel()
	if err != nil {
		t.amqpConn.Close()
		return err
	}

	t.amqpQ, err = t.amqpCh.QueueDeclare(
		queueName,
		true,
		false,
		false,
		false,
		nil,
	)

	if err != nil {
		return err
	}

	return nil
}

func (t *Translator) createMqttClient(mqttURL, clientID string) error {

	opts := mqtt.NewClientOptions().AddBroker(mqttURL).SetClientID(clientID)
	t.mqttClient = mqtt.NewClient(opts)

	if token := t.mqttClient.Connect(); token.Wait() && token.Error() != nil {
		slog.Error("Failed to connect to MQTT broker: %s", token.Error())
		return token.Error()
	}

	return nil
}

func (t *Translator) messageHandler(client mqtt.Client, msg mqtt.Message) {

	if err := t.amqpCh.Publish(
		"",
		t.amqpQ.Name,
		false,
		false,
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        t.mqttMsgProcessor(msg),
		},
	); err != nil {
		fmt.Printf("Failed to publish message: %s\n", err)
	}

}

// Subscribe subscribes to the specified MQTT topics with the given QoS levels.
// It uses the provided message handler to process incoming messages.
// It returns an error if any of the subscriptions fail.
func (t *Translator) Subscribe(topics map[string]byte) error {

	for key, qos := range topics {
		if token := t.mqttClient.Subscribe(key, qos, t.messageHandler); token.Wait() && token.Error() != nil {
			slog.Error("Failed to subscribe to topic %s: %s", key, token.Error())
			return token.Error()
		}
	}

	return nil
}

func (t *Translator) SetMqttMsgProcessor(handler MqttMsgProcessor) {
	t.mqttMsgProcessor = handler
}
