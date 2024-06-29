package main

import (
	"log"
	"time"

	amqp "github.com/rabbitmq/amqp091-go"

    "mqtt-client/mqtt"
)

const (
	amqpURL       = "amqp://guest:guest@localhost:5672/"
	mqttBrokerURL = "tcp://localhost:1883"
	exchangeName  = "iot-orchid"
	exchangeType  = "topic"
	queueName     = ""
	routingKey    = "mqtt.register"
	consumeWait   = 60 * time.Second // Time to wait for messages
)

func failOnError(err error, msg string) {
	if err != nil {
		log.Fatalf("%s: %s", msg, err)
	}
}



// Consumer struct
type Consumer struct {
	conn    *amqp.Connection
	channel *amqp.Channel
	queue   amqp.Queue
}

// NewConsumer creates a new consumer instance
func NewConsumer(amqpURL string) (*Consumer, error) {
	conn, err := amqp.Dial(amqpURL)
	if err != nil {
		return nil, err
	}

	ch, err := conn.Channel()
	if err != nil {
		conn.Close()
		return nil, err
	}

	return &Consumer{conn: conn, channel: ch}, nil
}

// Close closes the consumer connection and channel
func (c *Consumer) Close() {
	if c.channel != nil {
		c.channel.Close()
	}
	if c.conn != nil {
		c.conn.Close()
	}
}

// SetupQueue sets up the queue and binds it to the exchange
func (c *Consumer) SetupQueue(exchange, queueName, routingKey string) error {
	err := c.channel.ExchangeDeclare(
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

	q, err := c.channel.QueueDeclare(
		queueName,
		false,
		false,
		true,
		false,
		nil,
	)
	if err != nil {
		return err
	}

	err = c.channel.QueueBind(
		q.Name,
		routingKey,
		exchange,
		false,
		nil,
	)
	if err != nil {
		return err
	}

	c.queue = q
	return nil
}

// ConsumeMessages starts consuming messages from the queue
func (c *Consumer) ConsumeMessages(msgReceived chan<- bool) error {
	msgs, err := c.channel.Consume(
		c.queue.Name,
		"",
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		return err
	}

	go func() {
		log.Println("Consumer goroutine started...")
		for d := range msgs {
			log.Printf("Received a message: %s", d.Body)
			msgReceived <- true
			break
		}
	}()
	return nil
}

func main() {
	msgReceived := make(chan bool)

	// Start the consumer
	consumer, err := NewConsumer(amqpURL)
	failOnError(err, "Failed to create consumer")
	defer consumer.Close()

	err = consumer.SetupQueue(exchangeName, queueName, routingKey)
	failOnError(err, "Failed to setup queue")

	err = consumer.ConsumeMessages(msgReceived)
	failOnError(err, "Failed to start consuming messages")

	
    regHandler, err := mqtt.NewRegistrationHandler(amqpURL, mqttBrokerURL)

    failOnError(err, "Failed to create registration handler")

    go regHandler.Run()


	// Wait for a message or timeout
	select {
	case <-msgReceived:
		log.Println("Test successful: Message received")
	case <-time.After(consumeWait):
		log.Println("Test failed: No message received within timeout period")
	}
}
