use std::{env, sync::Arc};

use anyhow::Result;

use aerospike::{
    as_blob, as_key,
    Client, ClientPolicy, Error, Expiration, ResultCode, Value, WritePolicy, MapPolicy, 
    operations::{MapOrder, MapWriteMode, maps::{put, remove_by_rank_range_from, get_by_rank_range}}, as_val, MapReturnType
};
use apache_avro::{from_avro_datum, from_value, to_avro_datum, to_value, Schema};
use itertools::Itertools;

use super::{SchemasNames, SCHEMAS};
use crate::routes::{Action, UserProfilesRequest, UserProfilesResponse, UserTagRequest};

pub struct AerospikeStore {
    client: Client,
    write_policy: WritePolicy,
    map_policy: MapPolicy,
    namespace: String,
}

static USERS_TAG_SET: &str = "user_tag";

impl AerospikeStore {
    pub fn new() -> Self {
        let hosts = env::var("AEROSPIKE_HOSTS").expect("AEROSPIKE_HOSTS is not set");
        let namespace = env::var("AEROSPIKE_NAMESPACE").expect("AEROSPIKE_NAMESPACE is not set");

        let client_policy = ClientPolicy::default();
        let client = Client::new(&client_policy, &hosts).expect("Failed to connect to cluster");
        let write_policy = WritePolicy::new(0, Expiration::Never);
        let map_policy = MapPolicy::new(MapOrder::KeyOrdered, MapWriteMode::Update);

        Self {
            client,
            write_policy,
            map_policy,
            namespace,
        }
    }

    pub fn add_user_tag(&self, user_tag: Arc<UserTagRequest>) -> Result<()> {
        let key = as_key!(&self.namespace, &USERS_TAG_SET.to_owned(), &user_tag.cookie);
        let action: String = user_tag.action.to_string();

        let schema: &Schema = &SCHEMAS[SchemasNames::UserTag as usize];
        let datum = to_avro_datum(schema, to_value(&user_tag)?)?;
        let v_key = as_val!(-user_tag.time.timestamp_millis());
        let v_val = as_blob!(datum);

        let op = vec![
            put(&self.map_policy, &action, &v_key, &v_val),
            remove_by_rank_range_from(&action, 200, MapReturnType::None),
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
        let key = as_key!(&self.namespace, &USERS_TAG_SET.to_owned(), cookie);

        let action_buy = Action::BUY.to_string();
        let action_view = Action::VIEW.to_string();

        let limiti64 = req.limit.try_into().unwrap();
        let op = vec![
            get_by_rank_range(
                &action_buy,
                0,
                limiti64,
                aerospike::operations::maps::MapReturnType::Value,
            ),
            get_by_rank_range(
                &action_view,
                0,
                limiti64,
                aerospike::operations::maps::MapReturnType::Value,
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

            let schema: &Schema = &SCHEMAS[SchemasNames::UserTag as usize];
            let decoded_value = from_avro_datum(schema, &mut data.as_slice(), None);

            match decoded_value {
                Ok(value) => Ok(from_value::<UserTagRequest>(&value)?),
                Err(e) => Err(anyhow::Error::msg(format!(
                    "Error: invalid data type in user record: {:?}",
                    e
                ))),
            }
        };

        let process_fn = |tags: &Vec<Value>| {
            tags.iter()
                .map(v2struct)
                .filter_map(|r| r.ok())
                .filter(|r| req.time_range.is_in(r.time))
                .take(req.limit)
                .sorted_by(|a, b| b.time.cmp(&a.time))
                .collect()
        };

        let (buy_tags, view_tags) = match (
            record.bins.get(&action_buy).or(Some(&Value::List(vec![]))),
            record.bins.get(&action_view).or(Some(&Value::List(vec![]))),
        ) {
            (Some(Value::List(buy_tags)), Some(Value::List(view_tags))) => {
                (process_fn(buy_tags), process_fn(view_tags))
            }
            _ => unreachable!("Invalid data type in user record"),
        };

        Ok(UserProfilesResponse {
            cookie: cookie.to_owned(),
            views: view_tags,
            buys: buy_tags,
        })
    }
}
