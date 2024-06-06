package test

import (
	"log"
	"testing"

	amqp "github.com/rabbitmq/amqp091-go"
)

func TestSendMessage(t *testing.T) {
	// Connect to RabbitMQ server
	conn, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
	if err != nil {
		log.Fatalf("Failed to connect to RabbitMQ: %v", err)
	}
	defer conn.Close()

	// Create a channel
	ch, err := conn.Channel()
	if err != nil {
		log.Fatalf("Failed to open a channel: %v", err)
	}
	defer ch.Close()

	// Declare a queue
	queue, err := ch.QueueDeclare(
		"hello", // queue name
		false,        // durable
		false,        // delete when unused
		false,        // exclusive
		false,        // no-wait
		nil,          // arguments
	)
	if err != nil {
		log.Fatalf("Failed to declare a queue: %v", err)
	}

	// Publish a message to the queue
	message := "Hello, RabbitMQ!"
	err = ch.Publish(
		"",           // exchange
		queue.Name,   // routing key
		false,        // mandatory
		false,        // immediate
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        []byte(message),
		},
	)
	if err != nil {
		log.Fatalf("Failed to publish a message: %v", err)
	}

	// Consume the message from the queue
	msgs, err := ch.Consume(
		queue.Name, // queue
		"",         // consumer
		true,       // auto-ack
		false,      // exclusive
		false,      // no-local
		false,      // no-wait
		nil,        // arguments
	)
	if err != nil {
		log.Fatalf("Failed to consume a message: %v", err)
	}

	// Check if the message was received
	received := false
	for msg := range msgs {
		if string(msg.Body) == message {
			received = true
			break
		}
	}

	// Assert that the message was received
	if !received {
		t.Errorf("Expected message '%s' was not received", message)
	}
}