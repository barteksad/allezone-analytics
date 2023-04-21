mod kv;

use std::{fmt, sync::Arc};
use axum::{response::{Response, IntoResponse}, extract::{Query, Path}, Json, http::StatusCode, Extension};
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize, de::{Visitor, self, MapAccess}};

use self::kv::KVStore;

static DEFAULT_LIMIT: usize = 200;

#[derive(Clone)]
pub struct SharedKVStore(Arc<KVStore>);

#[utoipa::path(
  post,
  path="/user_tags",
  request_body=UserTagRequest,
  responses((status=204, description="User tag has been added successfully", body=UserTagResponse))
)]
pub async fn user_tags(Extension(SharedKVStore(kv_store)): Extension<SharedKVStore>, body: Json<UserTagRequest>) -> StatusCode {
  let kv_store_clone = kv_store.clone();
  let body_arc1 = Arc::new(body.0);
  let body_arc2 = body_arc1.clone();
  
  let agg_item_fut = tokio::task::spawn(async move {
    kv_store_clone.add_aggregates_item(body_arc1).await
  });
  let user_tag_fut = tokio::task::spawn_blocking(move || {
    kv_store.add_user_tag(body_arc2)
  });

  match tokio::try_join!(agg_item_fut, user_tag_fut) {
    Ok((Ok(_), Ok(_))) => StatusCode::NO_CONTENT,
    Ok((e1, e2)) => {
      tracing::error!("Error processing request: adding aggregate item result: {:?}, adding user tag result: {:?}", e1, e2);
      StatusCode::INTERNAL_SERVER_ERROR
    }
    Err(e) => {
      tracing::error!("Error processing request: {:?}", e);
      StatusCode::INTERNAL_SERVER_ERROR
    },
  }
}

#[utoipa::path(
  post,
  path="/user_profiles/{cookie}",
  params(
    ("time_range" = String, Path, ),
    ("limit", Path, )
  ),
  responses((status=200, description="User profiles has been fetched successfully", body=UserProfilesResponse))
)]
pub async fn user_profiles(
  Path(cookie): Path<String>, 
  Query(req): Query<UserProfilesRequest>,
  Extension(SharedKVStore(kv_store)): Extension<SharedKVStore>,   
  #[cfg(feature = "query-debug")] 
  body: Json<UserProfilesResponse>,
) -> Response {

  #[cfg(feature = "query-debug")]
  let time_range = req.time_range.clone();

  match tokio::task::spawn_blocking(move || {
    kv_store.get_user_tags(&cookie, &req)
  }).await {
    Ok(Ok(response)) => {
      let response_json = Json(response);
      // #[cfg(feature = "query-debug")]
      // {
      //   tracing::info!("\nTime range: {:?}\nResponse: {:?}\nExpected: {:?}",time_range, response_json, body);        
      // }
      return response_json.into_response();
    }
    Ok(Err(e)) => {
      tracing::error!("Error processing request: {:?}", e);
      return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    Err(e) => {
      tracing::error!("Error processing request: {:?}", e);
      return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
  }

  // #[cfg(feature = "query-debug")]
  // return body.into_response();  
}

#[utoipa::path(
  post,
  path="/aggregates",
  responses((status=200, description="Aggregates has been fetched successfully", body=AggregatesResponse))
)]
pub async fn aggregates(
  req: Query<AggregatesRequest>,
  Extension(SharedKVStore(kvStore)): Extension<SharedKVStore>,
  #[cfg(feature = "query-debug")]
  body: Json<AggregatesResponse>
) -> Response {
  #[cfg(feature = "query-debug")]
  {
    tracing::info!("\nRequest: {:?}\nExpected: {:?}", req, body);
    return body.into_response();
  }
  ().into_response()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTagRequest {
  time: DateTime<Utc>,
  cookie: String,

  country: String,
  device: Device,
  action: Action,
  origin: String,
  product_info: ProductInfo,  
}

#[derive(Serialize, Debug)]
struct UserTagResponse {}

#[derive(Deserialize, Debug)]
pub struct UserProfilesRequest {
  #[serde(deserialize_with = "deserialize_time_range")]
  time_range: TimeRange,
  #[serde(default="default_limit")]
  limit: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfilesResponse {
  cookie: String,
  views: Vec<UserTagRequest>,
  buys: Vec<UserTagRequest>,
}


#[derive(Deserialize, Debug)]
pub struct AggregatesRequest {
  #[serde(deserialize_with = "deserialize_time_range")]
  time_range: TimeRange,
  action: Action,
  #[serde(flatten)]
  aggregates: NAggregates,
  #[serde(default="option_default")]
  origin: Option<String>,
  #[serde(default="option_default")]
  brand_id: Option<String>,
  #[serde(default="option_default")]
  category_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregatesResponse {
  columns: Vec<String>,
  rows: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Device {
  PC,
  MOBILE,
  TV,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
  VIEW,
  BUY
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductInfo {
  product_id: usize,
  brand_id: String,
  category_id: String,
  price: i32,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "query-debug", derive(Clone))]
pub struct TimeRange {
  start: DateTime<Utc>,
  end: DateTime<Utc>,
}

impl SharedKVStore {
  pub fn new() -> SharedKVStore {
    SharedKVStore(Arc::new(KVStore::new()))
  }
}

impl TimeRange {
  pub fn is_in(&self, time: DateTime<Utc>) -> bool {
    self.start <= time && time < self.end
  }
}

#[derive(Serialize, Deserialize, Debug)]
enum Aggregates {
  COUNT,
  SUM_PRICE,
}

fn default_limit() -> usize {
  DEFAULT_LIMIT
}

fn option_default<T>() -> Option<T> {
  None
}

fn deserialize_time_range<'de, D>(deserializer: D) -> Result<TimeRange, D::Error>
where
  D: de::Deserializer<'de>,
{
  let visitor: TimeRange = TimeRange { start: DateTime::from(Utc::now()), end: DateTime::from(Utc::now())};
  deserializer.deserialize_str(visitor)
}

impl<'de> Visitor<'de> for TimeRange {
    type Value = TimeRange;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "struct TimeRange")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> 
    where
      E: de::Error, 
    {
      static FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.f";

      let mut split = s.split('_');
      let start = split.next();
      let end = split.next();
      match (start, end) {
        (Some(start), Some(end)) => {
          let start = Utc.datetime_from_str(start, FORMAT).map_err(de::Error::custom)?;
          let end = Utc.datetime_from_str(end, FORMAT).map_err(de::Error::custom)?;
          Ok(TimeRange { start: start, end: end })
        }
        _ =>  Err(de::Error::custom("invalid time range")),
      }
    }
}

#[derive(Debug)]
struct NAggregates(Vec<Aggregates>);

impl<'de> Deserialize<'de> for NAggregates {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: de::Deserializer<'de>,
  {
      struct MyVisitor;

      impl<'d> Visitor<'d> for MyVisitor {
          type Value = Vec<Aggregates>;

          fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
              f.write_str("a vec of aggregates")
          }

          fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
          where
              M: MapAccess<'d>,
          {
              let mut aggregates = Vec::new();
              while let Some((key, mut value)) = access.next_entry::<String, Aggregates>()? {
                  if key == "aggregates" {
                    aggregates.push(value);
                  } else {
                      return Err(de::Error::custom("invalid key"));
                  }
              }
              Ok(aggregates)
          }
      }
      Ok(NAggregates(deserializer.deserialize_map(MyVisitor)?))
  }
}

impl ToString for Action {
  fn to_string(&self) -> String {
    match self {
      Action::VIEW => "VIEW".to_string(),
      Action::BUY => "BUY".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use axum::{Json, response::IntoResponse};

use super::AggregatesResponse;


  #[test]
  fn ser_agg_rest() {
    let resp = AggregatesResponse {
      columns: vec!["a".to_string(), "b".to_string()],
      rows: vec![vec!["1".to_string(), "2".to_string()]],
    };

    let json = Json::<AggregatesResponse>(resp);
    println!("{:?}", json.into_response().body().to_owned());
  } 
}