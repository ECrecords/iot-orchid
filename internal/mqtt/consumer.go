package mqtt

import (
	"fmt"

	ampq "github.com/rabbitmq/amqp091-go"
)

type Consumer struct {
	ampqConn *ampq.Connection
	ampqCh   *ampq.Channel
	ampqQ    ampq.Queue
}

func NewConsumer(amqpURL string, amqpQueueName string) (*Consumer, error) {
	c := &Consumer{}

	if err := c.createWorkerQueue(amqpURL, amqpQueueName); err != nil {
		return nil, fmt.Errorf("failed to create worker queue: %v", err)
	}

	return c, nil
}

func (c *Consumer) createWorkerQueue(amqpURL, queueName string) error {
	var err error

	c.ampqConn, err = ampq.Dial(amqpURL)
	if err != nil {
		return err
	}

	c.ampqCh, err = c.ampqConn.Channel()
	if err != nil {
		c.ampqConn.Close()
		return err
	}

	c.ampqQ, err = c.ampqCh.QueueDeclare(
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

func (c *Consumer) Consume() (<-chan ampq.Delivery, error) {

	if c.ampqCh == nil {
		return nil, fmt.Errorf("channel is nil")
	}

	return c.ampqCh.Consume(
		c.ampqQ.Name,
		"",
		true,
		false,
		false,
		false,
		nil,
	)
}

