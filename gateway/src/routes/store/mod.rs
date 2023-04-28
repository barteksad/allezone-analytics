mod aerospike_store;
mod kafka_store;

use crate::routes::{UserProfilesRequest, UserProfilesResponse, UserTagRequest};

use std::sync::Arc;

use anyhow::Result;
use apache_avro::Schema;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub struct Store {
    aerospike_store: aerospike_store::AerospikeStore,
    kafka_store: kafka_store::KafkaStore,
}

static SCHEMAS: Lazy<[apache_avro::Schema; 3]> = Lazy::new(|| {
    [
        Schema::parse_str(include_str!("./schemas/UserTag.avsc")).unwrap(),
        Schema::parse_str(include_str!("./schemas/AggregatesItem.avsc")).unwrap(),
        Schema::parse_str(include_str!("./schemas/AggregatesPrice.avsc")).unwrap(),
    ]
});

enum SchemasNames {
    UserTag,
    AggregatesItem,
    AggregatesPrice,
}

#[derive(Serialize, Deserialize, Debug)]
struct AggregatesItem {
    time: i64,
    action: String,
    origin: String,
    brand_id: String,
    category_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AggregatesPrice {
    price: i32,
}

impl Store {
    pub fn new() -> Store {
		// force lazy static initialization
		Lazy::force(&SCHEMAS);
		
        Store {
            aerospike_store: aerospike_store::AerospikeStore::new(),
            kafka_store: kafka_store::KafkaStore::new(),
        }
    }

    pub fn add_user_tag(&self, user_tag: Arc<UserTagRequest>) -> Result<()> {
        self.aerospike_store.add_user_tag(user_tag)
    }

    pub fn get_user_tags(
        &self,
        cookie: &str,
        req: &UserProfilesRequest,
    ) -> Result<UserProfilesResponse> {
        self.aerospike_store.get_user_tags(cookie, req)
    }

    pub async fn add_aggregates_item(&self, user_tag: Arc<UserTagRequest>) -> Result<()> {
        self.kafka_store.add_aggregates_item(user_tag).await
    }
}

#[cfg(test)]
mod tests {
    use apache_avro::{from_avro_datum, to_avro_datum, to_value, Schema};
    use chrono::Utc;

    use crate::routes::{
        store::{SchemasNames, SCHEMAS},
        Action, Device, ProductInfo, UserTagRequest,
    };

    use super::{AggregatesItem, AggregatesPrice};

    #[test]
    fn test_schemas() {
        {
            let user_tag_request = UserTagRequest {
                time: Utc::now(),
                cookie: "abc".into(),
                country: "abc".into(),
                device: Device::MOBILE,
                action: Action::BUY,
                origin: "abc".into(),
                product_info: ProductInfo {
                    product_id: 1,
                    brand_id: "123".into(),
                    category_id: "abc".into(),
                    price: 1,
                },
            };
            let schema: &Schema = &SCHEMAS[SchemasNames::UserTag as usize];
            let datum = to_avro_datum(schema, to_value(&user_tag_request).unwrap()).unwrap();
            let decoded_value = from_avro_datum(schema, &mut datum.as_slice(), None).unwrap();
            let decoded_request = apache_avro::from_value::<UserTagRequest>(&decoded_value);
            assert!(decoded_request.is_ok());
            assert!(decoded_request.unwrap().time == user_tag_request.time);
        }

        {
            let aggregates_item = AggregatesItem {
                time: Utc::now().timestamp_millis(),
                action: Action::VIEW.to_string(),
                origin: "abc".into(),
                brand_id: "abc".into(),
                category_id: "abc".into(),
            };

            let schema: &Schema = &SCHEMAS[SchemasNames::AggregatesItem as usize];
            let datum = to_avro_datum(schema, to_value(&aggregates_item).unwrap()).unwrap();
            println!("{:?}, {:}", datum, datum.len());
        }

        {
            let aggregates_price = AggregatesPrice { price: 123 };

            let schema: &Schema = &SCHEMAS[SchemasNames::AggregatesPrice as usize];
            let datum = to_avro_datum(schema, to_value(&aggregates_price).unwrap()).unwrap();
            println!("{:?}, {:}", datum, datum.len());
        }
    }
}
