use std::fmt;
use axum::{response::{Response, IntoResponse}, extract::{Query, Path}, Json};
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize, de::{Visitor, self, MapAccess}};

static DEFAULT_LIMIT: usize = 200;

#[utoipa::path(
  post,
  path="/user_tags",
  request_body=UserTagRequest,
  responses((status=204, description="User tag has been added successfully", body=UserTagResponse))
)]
pub async fn user_tags(body: Json<UserTagRequest>) -> Response {
  println!("{:?}", body);
  ().into_response()
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
  cookie: Path<String>, 
  req: Query<UserProfilesRequest>,   
  #[cfg(feature = "query-debug")] 
  body: Json<UserProfilesResponse>
) -> Response {
  println!("{:?}", req);
  println!("{:?}", cookie);

  #[cfg(feature = "query-debug")]
  return body.into_response();
  
  ().into_response()
}

#[utoipa::path(
  post,
  path="/aggregates",
  responses((status=200, description="Aggregates has been fetched successfully", body=AggregatesResponse))
)]
pub async fn aggregates(
  req: Query<AggregatesRequest>,
  #[cfg(feature = "query-debug")]
  body: Json<AggregatesResponse>
) -> Response {
  println!("{:?}", req);
  #[cfg(feature = "query-debug")]
  return body.into_response();
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

#[derive(Serialize, Debug)]
struct UserProfilesResponse {
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

#[derive(Serialize, Debug)]
struct AggregatesResponse {
  columns: Vec<String>,
  rows: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Device {
  PC,
  MOBILE,
  TV,
}

#[derive(Serialize, Deserialize, Debug)]
enum Action {
  VIEW,
  BUY
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductInfo {
  product_id: String,
  brand_id: String,
  category_id: String,
  price: i32,
}

#[derive(Deserialize, Debug)]
struct TimeRange {
  start: DateTime<Utc>,
  end: DateTime<Utc>,
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
      static FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.%f";

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
