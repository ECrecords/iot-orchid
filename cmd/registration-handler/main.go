package main

import (
	"fmt"


	"github.com/ECrecords/iot-orchid/internal/mqtt"
)

func main() {
	fmt.Println("Registration Handler")

	c, err := mqtt.NewConsumer("amqp://guest:guest@localhost:5672/", "register-queue")

	if err != nil {
		panic(err)
	}

	deliveries, err := c.Consume()

	if err != nil {
		panic(err)
	}

	go func() {
		for d := range deliveries {
			fmt.Printf("Received a message: %s\n", d.Body)
		}
	}()

	forever := make(chan bool)
	<-forever

}
