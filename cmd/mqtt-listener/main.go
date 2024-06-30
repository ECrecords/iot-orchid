package main

import (
	"fmt"

	mqttBridge "github.com/ECrecords/iot-orchid/internal/mqtt"
	mqtt "github.com/eclipse/paho.mqtt.golang"
	viper "github.com/spf13/viper"
)

type Config struct {
	MQTT struct {
		URL      string `mapstructure:"url"`
		ClientID string `mapstructure:"client_id"`
		Topic    map[string]byte `mapstructure:"topic"`
	} `mapstructure:"mqtt"`
	AMQP struct {
		URL       string `mapstructure:"url"`
		WorkerQueue string `mapstructure:"worker_queue"`
	} `mapstructure:"amqp"`
}

func main() {
	
	viper.SetConfigType("yaml")
	viper.SetConfigName("config")
	viper.AddConfigPath(".")
	err := viper.ReadInConfig()
	if err != nil {
		panic(fmt.Errorf("Fatal error config file: %s \n", err))
	}

	var config Config
	err = viper.Unmarshal(&config)
	if err != nil {
		panic(fmt.Errorf("Fatal error config file: %s \n", err))
	}

	fmt.Print(config)

	forever := make(chan bool)

	// display config
	fmt.Printf("MQTT URL: %s\n", config.MQTT.URL)
	fmt.Printf("MQTT Client ID: %s\n", config.MQTT.ClientID)
	fmt.Printf("MQTT Topic: %s\n", config.MQTT.Topic)
	fmt.Printf("AMQP URL: %s\n", config.AMQP.URL)
	fmt.Printf("AMQP Worker Queue: %s\n", config.AMQP.WorkerQueue)


	t, err := mqttBridge.NewTranslator(config.AMQP.URL, config.AMQP.WorkerQueue, config.MQTT.URL, config.MQTT.ClientID)
	if err != nil {
		panic(err)
	}

	if err := t.Subscribe(config.MQTT.Topic); err != nil {
		panic(err)
	}

	t.SetMqttMsgProcessor(func(msg mqtt.Message) []byte {
		fmt.Printf("Received message: %s\n", msg.Payload())
		return msg.Payload()		
	})

	<-forever
}
