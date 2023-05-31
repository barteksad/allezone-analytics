use std::{env, sync::Arc};

use anyhow::Result;

use apache_avro::{to_avro_datum, to_value, Schema};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};

use crate::routes::UserTagRequest;

use super::{AggregatesItem, AggregatesPrice, SchemasNames, SCHEMAS};

pub struct KafkaStore {
    producer: FutureProducer,
    topic: String,
}

impl KafkaStore {
    pub fn new() -> Self {
        let hosts = env::var("KAFKA_HOSTS").expect("KAFKA_HOSTS is not set");
        let producer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", &hosts)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Kafka producer creation error");
        let topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC is not set");

        KafkaStore { producer, topic }
    }

    pub async fn add_aggregates_item(&self, user_tag: Arc<UserTagRequest>) -> Result<()> {
        let time_truncated_to_minute = user_tag.time.timestamp_millis() / (60 * 1000) * (60 * 1000);
        let aggregates_item = AggregatesItem {
            time: time_truncated_to_minute,
            action: user_tag.action.to_string(),
            origin: user_tag.origin.clone(),
            brand_id: user_tag.product_info.brand_id.clone(),
            category_id: user_tag.product_info.category_id.clone(),
        };
        let aggregates_price = AggregatesPrice {
            price: user_tag.product_info.price,
        };

        let item_schema: &Schema = &SCHEMAS[SchemasNames::AggregatesItem as usize];
        let price_schema: &Schema = &SCHEMAS[SchemasNames::AggregatesPrice as usize];

        let item_data = to_avro_datum(item_schema, to_value(aggregates_item)?)?;
        let price_data = to_avro_datum(price_schema, to_value(aggregates_price)?)?;

        match self
            .producer
            .send(
                FutureRecord::to(&self.topic)
                    .payload(&price_data)
                    .key(&item_data),
                Timeout::Never,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err((e, _)) => {
                tracing::error!("Error adding new user tag!: {:?}", e);
                Err(anyhow::Error::msg(e.to_string()))
            }
        }
    }
}
