package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	// amqp "github.com/rabbitmq/amqp091-go"
	mqtt "github.com/eclipse/paho.mqtt.golang"

	sq "github.com/Masterminds/squirrel"
	_ "github.com/jackc/pgx/v4/stdlib"
)

type RegisterResponse struct {
	DeviceId  string `json:"device_id"`
	ClusterId string `json:"cluster_id"`
}

type RegisterRequest struct {
	DeviceId string `json:"device_id"`
}

// func failOnError(err error, msg string) {
// 	if err != nil {
// 		log.Fatalf("%s: %s", msg, err)
// 	}
// }

func handleRegisterRequest(psql sq.StatementBuilderType, db *sql.DB, client mqtt.Client, msg mqtt.Message) {
	var registerRequest RegisterRequest

	err := json.Unmarshal(msg.Payload(), &registerRequest)
	if err != nil {
		log.Printf("Error parsing register request: %s", err)
		return
	}

	// Check if device is in database
	var deviceId, clusterId string

	err = psql.Select("id", "cluster_id").
		From("cluster_devices").
		Where(sq.Eq{"id": registerRequest.DeviceId}).
		RunWith(db).
		QueryRow().
		Scan(&deviceId, &clusterId)

	if err != nil {
		if err == sql.ErrNoRows {
			log.Printf("Device not found in database: %s", registerRequest.DeviceId)
			return // Or handle the scenario where device is not registered
		}
		log.Printf("Error checking if device exists: %s", err)
		return
	}

	// Create and send the response
	registerResponse := RegisterResponse {
		DeviceId:  deviceId,
		ClusterId: clusterId,
	}

	responseJson, err := json.Marshal(registerResponse)
	if err != nil {
		log.Printf("Error creating response: %s", err)
		return
	}

	responseTopic := fmt.Sprintf("device/register/response/%s", registerRequest.DeviceId)
	token := client.Publish(responseTopic, 0, false, responseJson)
	if token.Wait() && token.Error() != nil {
		log.Printf("Error sending response: %s", token.Error())
	}
}

func main() {
	// MQTT setup remains the same...
	opts := mqtt.NewClientOptions().AddBroker("tcp://localhost:1883").SetClientID("mqtt_sample_publisher")
	client := mqtt.NewClient(opts)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	// Database setup
	db, err := sql.Open("pgx", "host=localhost port=5432 user=postgres password=mysecretpassword dbname=postgres sslmode=disable")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()
	psql := sq.StatementBuilder.PlaceholderFormat(sq.Dollar)

	// Attach message callback
	client.Subscribe("device/register", 0, func(c mqtt.Client, m mqtt.Message) {
		handleRegisterRequest(psql, db, c, m)
	})

	// Remainder of the function...
	forever := make(chan bool)
	log.Printf(" [*] Waiting for messages. To exit press CTRL+C")
	<-forever
}
