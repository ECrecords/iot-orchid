package main

import (
	"context"
	"fmt"
	"time"

	// "log/slog"

	// mqtt "github.com/eclipse/paho.mqtt.golang"
	mqtt "github.com/eclipse/paho.mqtt.golang"
	amqp "github.com/rabbitmq/amqp091-go"
)

const MQTT_CLIENT_ID = "mqtt-gateway-service"
const CONTROL_QUEUE = "control"
const STATUS_QUEUE = "status"
const CONFIG_QUEUE = "config"
const REGISTER_QUEUE = "register"


type ControlMessage struct {
	DeviceID string `json:"device_id"`
	ClusterID string `json:"cluster_id"`
	Command string `json:"command"`
	
}


type MqttGatewayService struct {
	amqpConn *amqp.Connection
	mqttCli  mqtt.Client

	control *amqp.Channel
	status  *amqp.Channel
	config  *amqp.Channel
	register *amqp.Channel

	ctx    context.Context
	cancel context.CancelFunc
}

func (m *MqttGatewayService) LostConnectionHandler(c mqtt.Client, err error) {
	fmt.Println("Connection lost")
}

func NewMqttGatewayService(ctx context.Context, mqttBroker string, amqpBroker string) (*MqttGatewayService, error) {

	var err error

	m := &MqttGatewayService{}

	m.ctx, m.cancel = context.WithCancel(ctx)

	opts := mqtt.NewClientOptions().
		AddBroker(mqttBroker).
		SetClientID(MQTT_CLIENT_ID).
		SetCleanSession(true).
		SetConnectionLostHandler(m.LostConnectionHandler)

	mqttClient := mqtt.NewClient(opts)

	if token := mqttClient.Connect(); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}

	m.mqttCli = mqttClient 

	m.amqpConn, err = amqp.Dial(amqpBroker)
	if err != nil {
		return nil, err
	}

	m.control, err = createChannelWithQueue(m.amqpConn, CONTROL_QUEUE)
	if err != nil {
		return nil, err
	}

	m.status, err = createChannelWithQueue(m.amqpConn, STATUS_QUEUE)
	if err != nil {
		return nil, err
	}

	m.config, err = createChannelWithQueue(m.amqpConn, CONFIG_QUEUE) 
	if err != nil {
		return nil, err
	}

	m.register, err = createChannelWithQueue(m.amqpConn, REGISTER_QUEUE)
	if err != nil {
		return nil, err
	}

	return m, nil
}

func createChannelWithQueue(conn *amqp.Connection, qName string) (*amqp.Channel, error) {
	ch, err := conn.Channel()
	if err != nil {
		return nil, err
	}

	_, err = ch.QueueDeclare(
		qName, 		// name
		true,        // durable
		false,       // delete when unused
		false,       // exclusive
		false,       // no-wait
		nil,         // arguments
	)

	if err != nil {
		return nil, err
	}

	return ch, nil
}

func (m *MqttGatewayService) listenControl() {
	msgs, err := m.control.Consume(
		CONTROL_QUEUE, // queue
		"",            // consumer
		true,          // auto-ack
		false,         // exclusive
		false,         // no-local
		false,         // no-wait
		nil,           // args
	)

	if err != nil {
		fmt.Println("Failed to register a consumer")
		return
	}

	for {
		select {
		case <-m.ctx.Done():
			return
		case msg := <-msgs:
			go transmitControlMessage(m.mqttCli, msg)
		}
	}

}

func transmitControlMessage(cli mqtt.Client, msg amqp.Delivery) {


func (m *MqttGatewayService) Start() {
	go m.listenControl()
	go m.listenRegister()
}