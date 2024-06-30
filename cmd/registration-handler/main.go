package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"time"

	"github.com/ECrecords/iot-orchid/internal/mqtt"
	"github.com/spf13/viper"
)

const (
	amqpURI      = "amqp://guest:guest@localhost:5672/"
	queueName    = "registration-requests"
)

func main() {
	log.Println("Starting Registration Handler")

	viper.SetDefault("AMQP_URI", amqpURI)
	viper.SetDefault("QUEUE_NAME", queueName)

	viper.AutomaticEnv()

	amqpURI := viper.GetString("AMQP_URI")
	queueName := viper.GetString("QUEUE_NAME")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, os.Interrupt)


	c, err := mqtt.NewConsumer(amqpURI, queueName)
	if err != nil {
		log.Fatalf("Failed to create consumer: %v", err)
	}

	deliveries, err := c.Consume()
	if err != nil {
		log.Fatalf("Failed to start consuming messages: %v", err)
	}

	go func(parentCtx context.Context) {

		for {
			select {
			case <-parentCtx.Done():
				log.Println("Shutting down consumer.")
				return
			case d := <-deliveries:
				log.Printf("Received a message: %s", d.Body)
			}
		}
	}(ctx)

	
	sig := <-sigs
	log.Printf("Received signal: %v", sig)

	cancel()

	time.Sleep(1 * time.Second)
	log.Println("Exiting...")
}
