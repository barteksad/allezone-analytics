use crate::routes::{Action, UserProfilesRequest, UserProfilesResponse, UserTagRequest};

use std::{env, time::Duration, sync::Arc};

use aerospike::{
    as_blob, as_key,
    operations::lists::{
        get_by_index_range_count, insert, trim, ListOrderType, ListPolicy, ListWriteFlags,
    },
    Client, ClientPolicy, Error, Expiration, ResultCode, Value, WritePolicy,
};
use anyhow::Result;
use avro_rs::{from_value, Schema, Writer};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Serialize, Deserialize};

pub struct KVStore {
    as_client: Client,
    as_write_policy: WritePolicy,
    as_list_policy: ListPolicy,
    as_namespace: String,
    k_producer: FutureProducer,
    k_topic: String,
}

static SCHEMAS: Lazy<[avro_rs::Schema; 1]> = Lazy::new(|| {
    [
        Schema::parse_str(include_str!("./schemas/UserTag.avsc")).unwrap(),
    ]
});

enum SchemasNames {
    UserTag,
    AggregateItem,
}

#[derive(Serialize, Deserialize, Debug)]
struct AggregatesItem {
    time: DateTime<Utc>,
    action: Action,
    origin: String,
    brand_id: String,
    category_id: String,
    price: i32,
}

impl KVStore {
    pub fn new() -> KVStore {
        let client_policy = ClientPolicy::default();
        let as_hosts = env::var("AEROSPIKE_HOSTS").expect("AEROSPIKE_HOSTS is not set");
        let client = Client::new(&client_policy, &as_hosts).expect("Failed to connect to cluster");
        let write_policy = WritePolicy::new(0, Expiration::Never);
        let list_policy = ListPolicy::new(ListOrderType::Unordered, ListWriteFlags::Default);
        let namespace = env::var("AEROSPIKE_NAMESPACE").expect("AEROSPIKE_NAMESPACE is not set");
        
        let kafka_hosts = env::var("KAFKA_HOSTS").expect("KAFKA_HOSTS is not set");
        let k_producer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", &kafka_hosts)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Kafka producer creation error");
        let k_topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC is not set");

        KVStore {
            as_client: client,
            as_write_policy: write_policy,
            as_list_policy: list_policy,
            as_namespace: namespace,
            k_producer,
            k_topic,
        }
    }

    pub fn add_user_tag(&self, user_tag: Arc<UserTagRequest>) -> Result<()> {
        let key = as_key!(&self.as_namespace, &"user_tag".to_string(), &user_tag.cookie);
        let action: String = user_tag.action.to_string();

        let mut writer = Writer::new(&SCHEMAS[SchemasNames::UserTag as usize], Vec::<u8>::new());
        writer.append_ser(user_tag)?;
        let val = as_blob!(writer.into_inner()?);

        let op = vec![
            insert(&self.as_list_policy, &action, 0, &val),
            trim(&action, 0, 200),
        ];

        match self.as_client.operate(&self.as_write_policy, &key, &op) {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error adding new user tag!: {}", e);
                return Err(anyhow::Error::msg(e.to_string()));
            }
        }
    }

    pub fn get_user_tags(
        &self,
        cookie: &str,
        req: &UserProfilesRequest,
    ) -> Result<UserProfilesResponse> {
        let key = as_key!(&self.as_namespace, &"user_tag".to_string(), cookie);

        let action_buy = Action::BUY.to_string();
        let action_view = Action::VIEW.to_string();

        let op = vec![
            get_by_index_range_count(
                &action_buy,
                0,
                200,
                aerospike::operations::lists::ListReturnType::Values,
            ),
            get_by_index_range_count(
                &action_view,
                0,
                200,
                aerospike::operations::lists::ListReturnType::Values,
            ),
        ];

        let record = match self.as_client.operate(&self.as_write_policy, &key, &op) {
            Ok(r) => r,
            Err(Error(
                aerospike::errors::ErrorKind::ServerError(ResultCode::KeyNotFoundError),
                _,
            )) => {
                return Ok(UserProfilesResponse {
                    cookie: cookie.to_owned(),
                    views: vec![],
                    buys: vec![],
                })
            }
            Err(e) => {
                tracing::error!("Error: {}", e);
                return Err(anyhow::Error::msg(e.to_string()));
            }
        };

        let v2struct = |v: &Value| -> Result<UserTagRequest> {
            let data = match v {
                Value::Blob(data) => Ok(data),
                _ => {
                    tracing::error!("Error: invalid data type in user record");
                    Err(anyhow::Error::msg(
                        "Error: invalid data type in user record",
                    ))
                }
            }?;

            let mut reader =
                avro_rs::Reader::with_schema(&SCHEMAS[SchemasNames::UserTag as usize], &data[..])?;
            match reader.next() {
                Some(record) => Ok(from_value::<UserTagRequest>(&record?)?),
                None => Err(anyhow::Error::msg(
                    "Error: invalid data type in user record",
                )),
            }
        };

        let (buy_tags, view_tags) = match (
            record.bins.get(&action_buy).or(Some(&Value::List(vec![]))),
            record.bins.get(&action_view).or(Some(&Value::List(vec![]))),
        ) {
            (Some(Value::List(buy_tags)), Some(Value::List(view_tags))) => (
                buy_tags
                    .iter()
                    .map(v2struct)
                    .filter_map(|r| r.ok())
                    .filter(|r| req.time_range.is_in(r.time))
                    .take(req.limit)
                    .collect(),
                view_tags
                    .iter()
                    .map(v2struct)
                    .filter_map(|r| r.ok())
                    .filter(|r| req.time_range.is_in(r.time))
                    .take(req.limit)
                    .collect(),
            ),
            _ => unreachable!("Invalid data type in user record"),
        };

        Ok(UserProfilesResponse {
            cookie: cookie.to_owned(),
            views: view_tags,
            buys: buy_tags,
        })
    }

    pub async fn add_aggregates_item(&self, user_tag: Arc<UserTagRequest>) -> Result<()> {
        let aggregates_item = AggregatesItem {
            time: user_tag.time,
            action: user_tag.action.clone(),
            origin: user_tag.origin.clone(),
            brand_id: user_tag.product_info.brand_id.clone(),
            category_id: user_tag.product_info.category_id.clone(),
            price: user_tag.product_info.price.clone(),
        };

        let mut writer = Writer::new(&SCHEMAS[SchemasNames::AggregateItem as usize], Vec::<u8>::new());
        writer.append_ser(&aggregates_item)?;

        match self.k_producer.send(
    FutureRecord::to(&self.k_topic)
                .payload(&writer.into_inner()?)
                .key(&user_tag.cookie),
            Duration::from_secs(0)
        ).await {
            Ok(_) => Ok(()),
            Err((e, _)) => {
                tracing::error!("Error adding new user tag!: {:?}", e);
                Err(anyhow::Error::msg(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {

}
