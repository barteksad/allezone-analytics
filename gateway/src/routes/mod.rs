mod store;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::{DateTime, TimeZone, Utc};
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Serialize,
};
use tokio::time::Instant;
use std::{fmt, sync::Arc};

use metrics_exporter_prometheus::PrometheusHandle;

use self::store::Store;

static DEFAULT_USER_PROFILES_REQUEST_LIMIT: usize = 200;

#[derive(Clone)]
pub struct SharedStore(Arc<Store>);

/// Prometheus metrics scrape endpoint
#[utoipa::path(
	get,
	tag = "Text Generation Inference",
	path = "/metrics",
	responses((status = 200, description = "Prometheus Metrics", body = String))
)]
pub async fn metrics(prom_handle: Extension<PrometheusHandle>) -> String {
	prom_handle.render()
}

#[utoipa::path(
  post,
  path="/user_tags",
  request_body=UserTagRequest,
  responses(
	(status=204, description="User tag has been added successfully", body=UserTagResponse),
	(status=500, description="Internal server error")
  )
)]
#[tracing::instrument(
	skip_all,
	fields(
		total_time,
	))]
pub async fn user_tags(
    Extension(SharedStore(sstore)): Extension<SharedStore>,
    body: Json<UserTagRequest>,
) -> StatusCode {
	let start_time = Instant::now();
	metrics::increment_counter!("user_tags-offered-load");

    let sstore_clone = sstore.clone();
    let body_arc1 = Arc::new(body.0);
    let body_arc2 = body_arc1.clone();

    let agg_item_fut =
        tokio::task::spawn(async move { sstore_clone.add_aggregates_item(body_arc1).await });
    let user_tag_fut = tokio::task::spawn_blocking(move || sstore.add_user_tag(body_arc2));

 	match tokio::try_join!(agg_item_fut, user_tag_fut) {
        Ok((Ok(_), Ok(_))) => {
			let total_time = start_time.elapsed();
			metrics::histogram!("user_tags-load-time-successful", total_time.as_millis() as f64);
			metrics::increment_counter!("user_tags-successful-load");
			StatusCode::NO_CONTENT
		},
        Ok((e1, e2)) => {
			let total_time = start_time.elapsed();
			metrics::histogram!("user_tags-load-time-failed", total_time.as_millis() as f64);
			metrics::increment_counter!("user_tags-failed-load");
            tracing::error!("Error processing request: adding aggregate item result: {:?}, adding user tag result: {:?}", e1, e2);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Err(e) => {
			let total_time = start_time.elapsed();
			metrics::histogram!("user_tags-load-time-failed-internal", total_time.as_millis() as f64);
			metrics::increment_counter!("user_tags-failed-load-internal");
            tracing::error!("Error processing request: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
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
#[tracing::instrument(
	skip_all,
	fields(
		total_time,
	))]
pub async fn user_profiles(
    Path(cookie): Path<String>,
    Query(req): Query<UserProfilesRequest>,
    Extension(SharedStore(sstore)): Extension<SharedStore>,
    #[cfg(feature = "query-debug")] 
	body: Json<UserProfilesResponse>,
) -> Response {
	let start_time = Instant::now();
	metrics::increment_counter!("user_profiles-offered-load");

    match tokio::task::spawn_blocking(move || sstore.get_user_tags(&cookie, &req)).await {
        Ok(Ok(response)) => {
            let response_json = Json(response);
			let total_time = start_time.elapsed();
			metrics::histogram!("user_profiles-load-time-successful", total_time.as_millis() as f64);
			metrics::increment_counter!("user_profiles-successful-load");
            #[cfg(feature = "query-debug")]
            {
              if body.0 != response_json.0 {
                tracing::error!("Prof: Request: {:?}, Response: {:?}", body.0, response_json.0);
              }
            }
            return response_json.into_response();
        }
        Ok(Err(e)) => {
			let total_time = start_time.elapsed();
			metrics::histogram!("user_profiles-load-time-failed", total_time.as_millis() as f64);
			metrics::increment_counter!("user_profiles-failed-load");
            tracing::error!("Error processing request: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        Err(e) => {
			let total_time = start_time.elapsed();
			metrics::histogram!("user_profiles-load-time-failed-internal", total_time.as_millis() as f64);
			metrics::increment_counter!("user_profiles-failed-load-internal");
            tracing::error!("Error processing request: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
}

#[utoipa::path(
  post,
  path="/aggregates",
  responses((status=200, description="Aggregates has been fetched successfully", body=AggregatesResponse))
)]
#[tracing::instrument(
	skip_all,
	fields(
		total_time,
	))]
pub async fn aggregates(
    req: Query<AggregatesRequest>,
    Extension(SharedStore(sstore)): Extension<SharedStore>,
    #[cfg(feature = "query-debug")] body: Json<AggregatesResponse>,
) -> Response {
	let start_time = Instant::now();
	metrics::increment_counter!("aggregates-offered-load");

	match sstore.get_aggregates(&req).await {
		Ok(response) => {
			let response_json = Json(response);
			let total_time = start_time.elapsed();
			metrics::histogram!("aggregates-load-time-successful", total_time.as_millis() as f64);
			metrics::increment_counter!("aggregates-successful-load");
            #[cfg(feature = "query-debug")]
            {
              if body.0 != response_json.0 {
                tracing::error!("Agg:, Request: {:?}, Response: {:?}", body.0, response_json.0);
              }
            }
			return response_json.into_response();
		}
		Err(e) => {
			let total_time = start_time.elapsed();
			metrics::histogram!("aggregates-load-time-failed", total_time.as_millis() as f64);
			metrics::increment_counter!("aggregates-failed-load");
			tracing::error!("Error processing request: {:?}", e);
			return StatusCode::INTERNAL_SERVER_ERROR.into_response();
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "query-debug", derive(PartialEq))]
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
    #[serde(default = "default_limit")]
    limit: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "query-debug", derive(PartialEq))]
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
    #[serde(default = "option_default")]
    origin: Option<String>,
    #[serde(default = "option_default")]
    brand_id: Option<String>,
    #[serde(default = "option_default")]
    category_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "query-debug", derive(PartialEq))]
pub struct AggregatesResponse {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "query-debug", derive(PartialEq))]
enum Device {
    PC,
    MOBILE,
    TV,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "query-debug", derive(PartialEq))]
pub enum Action {
    VIEW,
    BUY,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "query-debug", derive(PartialEq))]
struct ProductInfo {
    product_id: usize,
    brand_id: String,
    category_id: String,
    price: i32,
}

#[derive(Deserialize, Debug)]
pub struct TimeRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl SharedStore {
    pub async fn new() -> SharedStore {
        SharedStore(Arc::new(Store::new().await))
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
    #[allow(non_camel_case_types)]
    SUM_PRICE,
}

fn default_limit() -> usize {
    DEFAULT_USER_PROFILES_REQUEST_LIMIT
}

fn option_default<T>() -> Option<T> {
    None
}

fn deserialize_time_range<'de, D>(deserializer: D) -> Result<TimeRange, D::Error>
where
    D: de::Deserializer<'de>,
{
    let visitor: TimeRange = TimeRange {
        start: DateTime::from(Utc::now()),
        end: DateTime::from(Utc::now()),
    };
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
                let start = Utc
                    .datetime_from_str(start, FORMAT)
                    .map_err(de::Error::custom)?;
                let end = Utc
                    .datetime_from_str(end, FORMAT)
                    .map_err(de::Error::custom)?;
                Ok(TimeRange {
                    start: start,
                    end: end,
                })
            }
            _ => Err(de::Error::custom("invalid time range")),
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
                while let Some((key, value)) = access.next_entry::<String, Aggregates>()? {
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
    use axum::{response::IntoResponse, Json};

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
