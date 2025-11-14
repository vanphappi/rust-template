pub mod message_queue;

#[cfg(feature = "mq-kafka")]
pub mod kafka;

#[cfg(feature = "mq-rabbitmq")]
pub mod rabbitmq;

#[cfg(feature = "mq-nats")]
pub mod nats_client;

pub use message_queue::{Message, MessageQueue, MessageHandler};

#[cfg(feature = "mq-kafka")]
pub use kafka::KafkaProducer;

#[cfg(feature = "mq-rabbitmq")]
pub use rabbitmq::RabbitMQClient;

#[cfg(feature = "mq-nats")]
pub use nats_client::NatsClient;

