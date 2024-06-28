package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	amqp "github.com/rabbitmq/amqp091-go"

    "mqtt-client/mqtt"
)

func failOnError(err error, msg string) {
	if err != nil {
		log.Panicf("%s: %s", msg, err)
	}
}

func startRabbitMQConsumer() {

	log.Println("Starting RabbitMQ consumer...")

    conn, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
    failOnError(err, "Failed to connect to RabbitMQ")
    defer conn.Close()

    ch, err := conn.Channel()
    failOnError(err, "Failed to open a channel")
    defer ch.Close()

    err = ch.ExchangeDeclare(
        "iotorchid.mqtt",
        "topic",
        true,
        false,
        false,
        false,
        nil,
    )
    failOnError(err, "Failed to declare an exchange")

    q, err := ch.QueueDeclare(
        "mqtt_registration_queue", // Named queue
        false,
        false,
        true,
        false,
        nil,
    )
    failOnError(err, "Failed to declare a queue")

    err = ch.QueueBind(
        q.Name,
        "mqtt.registration",
        "iot-orchid.mqtt",
        false,
        nil,
    )
    failOnError(err, "Failed to bind a queue")

    msgs, err := ch.Consume(
        q.Name,
        "",
        true,
        false,
        false,
        false,
        nil,
    )
    failOnError(err, "Failed to register a consumer")

    go func() {
        for d := range msgs {
            log.Printf("Received a message: %s", d.Body)
        }
    }()
}


func main() {
	// Handle OS signals for graceful shutdown
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

	
	rm, err := mqtt.NewRegistrationHandler("amqp://guest:guest@localhost:5672/", "tcp://localhost:1883")
	failOnError(err, "Failed to create RegistrationHandler")
	
	err = rm.Start()
	failOnError(err, "Failed to start RegistrationHandler")
	
	log.Println("Service started. Waiting for messages...")
	
	go startRabbitMQConsumer()
	
	// Wait for interrupt signal to gracefully shutdown the service
	<-sigChan
	log.Println("Interrupt signal received, shutting down...")
	
	// Close the RegistrationHandler resources
	rm.Close()
	log.Println("Service shut down gracefully.")
}
