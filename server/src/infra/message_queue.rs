use lapin::{
    BasicProperties, Channel,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{FieldTable, ShortString},
};
use serde_json::Value;

use crate::{constants::SCANS_QUEUE, infra::message_queue::error::MqError};

pub mod error;
pub mod publisher;

pub struct RabbitMqConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub vhost: String,
}

impl RabbitMqConfig {
    pub fn uri(&self) -> String {
        format!(
            "amqp://{username}:{password}@{host}:{port}/{vhost}",
            username = urlencoding::encode(&self.username),
            password = urlencoding::encode(&self.password),
            host = self.host,
            port = self.port,
            vhost = urlencoding::encode(&self.vhost),
        )
    }
}

pub trait MqProvider {
    fn publish(
        &self,
        channel: ShortString,
        message: Value,
    ) -> impl Future<Output = Result<(), MqError>> + Send;
}

pub struct RabbitMqProvider {
    channel: Channel,
}

impl RabbitMqProvider {
    pub async fn build(channel: Channel) -> Result<Self, MqError> {
        channel
            .queue_declare(
                SCANS_QUEUE.into(),
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await
            .or(Err(MqError::BuildError))?;
        Ok(Self { channel })
    }
}

impl MqProvider for RabbitMqProvider {
    async fn publish(&self, channel: ShortString, message: Value) -> Result<(), MqError> {
        self.channel
            .basic_publish(
                "".into(),
                channel,
                BasicPublishOptions::default(),
                serde_json::to_vec(&message)?.as_slice(),
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_no_escape() {
        // Arrange
        let expected_username = String::from("test");
        let expected_password = String::from("test");
        let expected_host = String::from("localhost");
        let expected_port: u16 = 5672;
        let expected_vhost = String::from("");
        let rabbit_mq_config = RabbitMqConfig {
            username: expected_username.clone(),
            password: expected_password.clone(),
            host: expected_host.clone(),
            port: expected_port,
            vhost: expected_vhost,
        };
        let expected_uri = format!(
            "amqp://{}:{}@{}:{}/",
            expected_username, expected_password, expected_host, expected_port
        );

        // Act
        let actual_uri = rabbit_mq_config.uri();

        // Assert
        assert_eq!(actual_uri, expected_uri);
    }

    #[test]
    fn test_uri_no_bad_chars() {
        // Arrange
        let expected_username = String::from("t@st");
        let expected_password = String::from("t@st");
        let expected_host = String::from("localhost");
        let expected_port: u16 = 5672;
        let expected_vhost = String::from("/");
        let rabbit_mq_config = RabbitMqConfig {
            username: expected_username,
            password: expected_password,
            host: expected_host,
            port: expected_port,
            vhost: expected_vhost,
        };
        let expected_uri = String::from("amqp://t%40st:t%40st@localhost:5672/%2F");
        // Act
        let actual_uri = rabbit_mq_config.uri();

        // Assert
        assert_eq!(actual_uri, expected_uri);
    }
}
