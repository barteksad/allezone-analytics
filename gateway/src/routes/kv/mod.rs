use crate::routes::{Action, UserProfilesRequest, UserProfilesResponse, UserTagRequest};

use std::env;

use aerospike::{
    as_blob, as_key,
    operations::lists::{
        get_by_index_range_count, insert, trim, ListOrderType, ListPolicy, ListWriteFlags,
    },
    Client, ClientPolicy, Error, Expiration, ResultCode, Value, WritePolicy,
};
use anyhow::Result;
use avro_rs::{from_value, Schema, Writer};
use once_cell::sync::Lazy;

pub struct KVStore {
    client: Client,
    write_policy: WritePolicy,
    list_policy: ListPolicy,
    namespace: String,
}

static SCHEMAS: Lazy<[avro_rs::Schema; 1]> = Lazy::new(|| {
    [
        Schema::parse_str(include_str!("./schemas/UserTag.avsc")).unwrap(),
        // Schema::parse_str(include_str!("./schemas/Aggregate.avsc")).unwrap(),
    ]
});

enum SchemasNames {
    UserTag,
    Aggregate,
}

impl KVStore {
    pub fn new() -> KVStore {
        let client_policy = ClientPolicy::default();
        let hosts = env::var("AEROSPIKE_HOSTS").expect("AEROSPIKE_HOSTS is not set");
        let client = Client::new(&client_policy, &hosts).expect("Failed to connect to cluster");
        let write_policy = WritePolicy::new(0, Expiration::Never);
        let list_policy = ListPolicy::new(ListOrderType::Unordered, ListWriteFlags::Default);
        let namespace = env::var("AEROSPIKE_NAMESPACE").expect("AEROSPIKE_NAMESPACE is not set");

        KVStore {
            client,
            write_policy,
            list_policy,
            namespace,
        }
    }

    pub fn add_user_tag(&self, user_tag: &UserTagRequest) -> Result<()> {
        let key = as_key!(&self.namespace, &"user_tag".to_string(), &user_tag.cookie);
        let action: String = user_tag.action.to_string();

        let mut writer = Writer::new(&SCHEMAS[SchemasNames::UserTag as usize], Vec::<u8>::new());
        writer.append_ser(user_tag)?;
        let val = as_blob!(writer.into_inner()?);

        let op = vec![
            insert(&self.list_policy, &action, 0, &val),
            trim(&action, 0, 200),
        ];

        match self.client.operate(&self.write_policy, &key, &op) {
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
        let key = as_key!(&self.namespace, &"user_tag".to_string(), cookie);

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

        let record = match self.client.operate(&self.write_policy, &key, &op) {
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
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::routes::{Action, Device, ProductInfo};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Simple1 {
        time: DateTime<Utc>,
        cookie: String,

        country: String,
        device: Device,
        action: Action,
        origin: String,
        product_info: ProductInfo,
    }

    // let schema = r#"
    //     {
    //       "namespace": "allezone-analytics",
    //       "type": "record",
    //       "name": "usertag",
    //       "fields" : [
    //         {"name": "time", "type": "string"},
    //         {"name": "cookie", "type": "string"},
    //         {"name": "device", "type": "string"},
    //         {"name": "action", "type": "string"},
    //         {"name": "origin", "type": "string"},
    //         {"name": "product_info", "type": "record", "fields": [
    //           {"name": "product_id", "type": "long"},
    //           {"name": "brand_id", "type": "string"},
    //           {"name": "category_id", "type": "string"},
    //           {"name": "price", "type": "int"}
    //           ]}
    //           ]
    //         }"#;

    #[test]
    fn schemas() {
        let schema = r#"
        {
          "namespace": "allezone-analytics",
          "type": "record",
          "name": "usertag",
          "fields" : [
            {"name": "time", "type": "string"},
            {"name": "cookie", "type": "string"},
            {"name": "country", "type": "string"},
            {"name": "device", "type": "enum", "symbols": ["PC", "MOBILE", "TV"]},
            {"name": "action", "type": "enum", "symbols": ["VIEW", "BUY"]},
            {"name": "origin", "type": "string"},
            {"name": "product_info", "type": "record", "fields": [
              {"name": "product_id", "type": "long"},
              {"name": "brand_id", "type": "string"},
              {"name": "category_id", "type": "string"},
              {"name": "price", "type": "int"}
            ]}
              ]
            }"#;

        let x = Simple1 {
            time: Utc::now(),
            cookie: "123".to_string(),
            country: "RU".to_string(),
            device: Device::PC,
            action: Action::BUY,
            origin: "google".to_string(),
            product_info: ProductInfo {
                product_id: 123,
                brand_id: "123".to_string(),
                category_id: "123".to_string(),
                price: 123,
            },
        };

        let schema = avro_rs::Schema::parse_str(schema).unwrap();
        let mut writer = avro_rs::Writer::new(&schema, Vec::new());
        writer.append_ser(&x).unwrap();
        let data = writer.into_inner().unwrap();
        let mut reader = avro_rs::Reader::with_schema(&schema, &data[..]).unwrap();
        let value = reader.next().unwrap().unwrap();
        let x: Simple1 = avro_rs::from_value(&value).unwrap();
        println!("{:?}", x);
    }
}
