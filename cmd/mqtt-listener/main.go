package main

import (
	"fmt"
	mqttBridge "github.com/ECrecords/iot-orchid/internal/mqtt"

	mqtt "github.com/eclipse/paho.mqtt.golang"
)

func main() {
	

	forever := make(chan bool)
	topics := map[string]byte{
		"mqtt/register": 0,
	}

	t, err := mqttBridge.NewTranslator("amqp://guest:guest@localhost:5672/", "register-queue", "tcp://localhost:1883", "register-producer")
	if err != nil {
		panic(err)
	}

	if err := t.Subscribe(topics); err != nil {
		panic(err)
	}

	t.SetMqttMsgProcessor(func(msg mqtt.Message) []byte {
		fmt.Printf("Received message: %s\n", msg.Payload())
		return msg.Payload()		
	})

	<-forever
}
