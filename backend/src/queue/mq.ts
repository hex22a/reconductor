import amqp from 'amqplib';

export type MqProvider = typeof amqp;
export const mq = amqp;
