use crate::routes::{UserTagRequest, UserProfilesResponse, Action, UserProfilesRequest};

use std::env;

use anyhow::Result;
use aerospike::{Client, ClientPolicy, operations::lists::{ListPolicy, ListOrderType, ListWriteFlags, insert, trim, get_by_index_range_count}, as_key, as_val, WritePolicy, Expiration, Value};
use avro_rs::{Schema, Writer, from_value};
use once_cell::sync::Lazy;

pub struct KVStore {
  client: Client,
  write_policy: WritePolicy,
  list_policy: ListPolicy,
  namespace: String,
}

static SCHEMAS: Lazy<[avro_rs::Schema; 1]> = Lazy::new(|| [
  Schema::parse_str(include_str!("./schemas/UserTag.avsc")).unwrap(),
  // Schema::parse_str(include_str!("./schemas/Aggregate.avsc")).unwrap(),
]);

enum SchemasNames {
  UserTag,
  Aggregate,
}

impl KVStore {
  pub fn new() -> KVStore {
    let client_policy = ClientPolicy::default();
    let hosts = env::var("AEROSPIKE_HOSTS")
        .unwrap_or(String::from("127.0.0.1:3000"));
    let client = Client::new(&client_policy, &hosts)
      .expect("Failed to connect to cluster");
    let write_policy = WritePolicy::new(0, Expiration::Never);
    let list_policy = ListPolicy::new(ListOrderType::Unordered, ListWriteFlags::Default);
    let namespace = env::var("AEROSPIKE_NAMESPACE")
      .unwrap_or(String::from("test"));

    KVStore { client, write_policy, list_policy, namespace }
  }

  pub fn add_user_tag(&self, user_tag: &UserTagRequest) -> Result<()> {
    let key = as_key!(&self.namespace, &"user_tag".to_string(), &user_tag.cookie);
    let action: String = user_tag.action.to_string(); 
    
    let mut writer = Writer::new(&SCHEMAS[SchemasNames::UserTag as usize], Vec::new());
    writer.append_ser(user_tag)?;
    let val = as_val!(writer.into_inner()?);
    
    let op = vec![
      insert(&self.list_policy, &action, 0, &val),
      trim(&action,0, 200),
    ];

    match self.client.operate(&self.write_policy, &key, &op) {
      Ok(_) => Ok(()),
      Err(e) => {
        tracing::error!("Error: {}", e);
        return Err(anyhow::Error::msg(e.to_string()));
      }
    }
  }

  pub fn get_user_tags(&self, cookie: &str, req: &UserProfilesRequest) -> Result<UserProfilesResponse> {
    let key = as_key!(&self.namespace, &"user_tag".to_string(), cookie);

    let action_buy = Action::BUY.to_string();
    let action_view = Action::VIEW.to_string();
    
    let op = vec![
      get_by_index_range_count(&action_buy, 0, req.limit.try_into().unwrap(), aerospike::operations::lists::ListReturnType::Values),
      get_by_index_range_count(&action_view, 0, req.limit.try_into().unwrap(), aerospike::operations::lists::ListReturnType::Values)
    ];

    let record = match self.client.operate(&self.write_policy, &key, &op) {
      Ok(r) => r,
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
          Err(anyhow::Error::msg("Error: invalid data type in user record"))
        }
      }?;
      let mut reader = avro_rs::Reader::with_schema(&SCHEMAS[SchemasNames::UserTag as usize], &data[..])?;
      match reader.next() {
        Some(record) => Ok(from_value::<UserTagRequest>(&record?)?),
        None => Err(anyhow::Error::msg("Error: invalid data type in user record")),
      }
    };

    let (buy_tags, view_tags) = match (record.bins.get(&action_buy), record.bins.get(&action_view)) {
      (Some(Value::List(buy_tags)), Some(Value::List(view_tags))) => (
        buy_tags
          .iter()
          .map(v2struct)
          .filter_map(|r| r.ok())
          .filter(|r| req.time_range.is_in(r.time))
          .collect(), 
        view_tags
          .iter()
          .map(v2struct)
          .filter_map(|r| r.ok())
          .collect()
      ),
      _ => return Err(anyhow::Error::msg("Error: no tags found")),
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

  #[test]
  fn schemas() {
  }
}