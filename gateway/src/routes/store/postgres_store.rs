use std::{collections::HashMap, env, iter::zip};

use anyhow::Result;
use chrono::{Duration, NaiveDateTime};
use itertools::Itertools;
use tokio_postgres::{tls, types::ToSql, Client};

use crate::routes::{Aggregates, AggregatesRequest, AggregatesResponse};

pub struct PostgresStore {
    client: Client,
}

static AGGREGATES_QUERY: &str = "
SELECT time, action SELECT_PLACEHOLDER
FROM aggregates
WHERE time >= $1 and time < $2
	AND action = $3
	ORIGIN_PLACEHOLDER
	BRAND_PLACEHOLDER
	CATEGORY_PLACEHOLDER
GROUP BY time, action GROUP_BY_PLACEHOLDER
ORDER BY time";

impl PostgresStore {
    pub async fn new() -> Self {
        let host = env::var("DB_HOST").expect("DB_HOST is not set");
        let port = env::var("DB_PORT").expect("DB_PORT is not set");
        let user = env::var("DB_USER").expect("DB_USER is not set");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD is not set");
        let dbname = env::var("DB_NAME").expect("DB_NAME is not set");

        let mut config = tokio_postgres::Config::new();
        config.host(&host);
        config.port(port.parse::<u16>().unwrap());
        config.user(&user);
        config.password(&password);
        config.dbname(&dbname);

        let (client, connection) = config
            .connect(tls::NoTls)
            .await
            .expect("Failed to connect to Postgres");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        PostgresStore { client }
    }

    pub async fn get_aggregates(&self, req: &AggregatesRequest) -> Result<AggregatesResponse> {
        let query = build_query(req);

        let mut columns: Vec<String> = vec!["1m_bucket".to_string(), "action".to_string()];

        let start_dt = req.time_range.start.naive_utc();
        let end_dt = req.time_range.end.naive_utc();
        let action_str = req.action.to_string();

        let mut params: Vec<&(dyn ToSql + Sync)> = vec![&start_dt, &end_dt, &action_str];
        let mut row_values_names = vec![&action_str];

        if let Some(origin) = &req.origin {
            params.push(origin);
            row_values_names.push(origin);
            columns.push("origin".to_string());
        }

        if let Some(brand_id) = &req.brand_id {
            params.push(brand_id);
            row_values_names.push(brand_id);
            columns.push("brand_id".to_string());
        }

        if let Some(category_id) = &req.category_id {
            params.push(category_id);
            row_values_names.push(category_id);
            columns.push("category_id".to_string());
        }

        for aggregate in &req.aggregates.0 {
            match aggregate {
                Aggregates::COUNT => columns.push("count".to_string()),
                Aggregates::SUM_PRICE => columns.push("sum_price".to_string()),
            }
        }

        let query_rows = match self.client.query(query.as_str(), &params).await {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!("Error getting aggregates: {}", e);
                println!("{:?}", e);
                return Err(e.into());
            }
        };

        let rows = query_rows
            .iter()
            .map(|row| {
                zip(0..columns.len(), &columns)
                    .map(|(i, col_name)| match (i, col_name) {
                        (0, _) => {
                            let dt: NaiveDateTime = row.get(i);
                            return dt.format("%Y-%m-%dT%H:%M:%S").to_string();
                        }
                        (_, c) if c == "count" || c == "sum_price" => {
                            row.get::<_, i64>(i).to_string()
                        }
                        _ => row.get(i),
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let n_rows_gt = req
            .time_range
            .end
            .signed_duration_since(req.time_range.start)
            .num_minutes();

        let full_rows: HashMap<String, Vec<String>> = (0..n_rows_gt)
            .map(|i| {
                let dt = req.time_range.start + Duration::minutes(i);
                let dt_formatted = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
                let mut row: Vec<String> = Vec::new();
                row.extend(row_values_names.iter().map(|v| v.to_string()));
                row.extend(vec!["0".to_string(); req.aggregates.0.len()]);
                (dt_formatted, row)
            })
            .chain(rows.into_iter().map(|r| (r[0].to_owned(), r[1..].to_vec())))
            .collect();

        let rows = full_rows
            .into_iter()
            .map(|(k, v)| {
                let mut row = vec![k];
                row.extend(v);
                row
            })
            .sorted_by(|a, b| a[0].cmp(&b[0]))
            .collect::<Vec<Vec<String>>>();

        Ok(AggregatesResponse { columns, rows })
    }
}

fn build_query(req: &AggregatesRequest) -> String {
    let mut query = AGGREGATES_QUERY.to_string();

    let mut i = 4;
    let mut select = String::new();
    let mut group_by = String::new();

    if req.origin.is_some() {
        query = query.replace("ORIGIN_PLACEHOLDER", &format!("AND origin = ${}", i));
        select.push_str(", origin");
        group_by.push_str(", origin");
        i += 1;
    } else {
        query = query.replace("ORIGIN_PLACEHOLDER\n", "");
    }

    if req.brand_id.is_some() {
        query = query.replace("BRAND_PLACEHOLDER", &format!("AND brand_id = ${}", i));
        select.push_str(", brand_id");
        group_by.push_str(", brand_id");
        i += 1;
    } else {
        query = query.replace("BRAND_PLACEHOLDER\n", "");
    }

    if req.category_id.is_some() {
        query = query.replace("CATEGORY_PLACEHOLDER", &format!("AND category_id = ${}", i));
        select.push_str(", category_id");
        group_by.push_str(", category_id");
    } else {
        query = query.replace("CATEGORY_PLACEHOLDER\n", "");
    }

    for aggregate in &req.aggregates.0 {
        match aggregate {
            Aggregates::COUNT => select.push_str(", SUM(count)::BIGINT"),
            Aggregates::SUM_PRICE => select.push_str(", SUM(sum)::BIGINT"),
        }
    }

    query = query.replace(" SELECT_PLACEHOLDER", &select);
    query = query.replace(" GROUP_BY_PLACEHOLDER", &group_by);

    query
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::routes::{Action, Aggregates, NAggregates, TimeRange};

    use super::*;

    #[test]
    fn test_build_query() {
        let req = AggregatesRequest {
            time_range: TimeRange {
                start: Utc::now(),
                end: Utc::now(),
            },
            action: Action::VIEW,
            origin: Some("origin".to_string()),
            brand_id: Some("brand_id".to_string()),
            category_id: Some("category_id".to_string()),
            aggregates: NAggregates(vec![Aggregates::COUNT, Aggregates::SUM_PRICE]),
        };

        let query = build_query(&req);
        assert_eq!(
            query.replace("\t", ""),
            "
SELECT time, action, origin, brand_id, category_id, SUM(count)::BIGINT, SUM(sum)::BIGINT
FROM aggregates
WHERE time >= $1 and time < $2
	AND action = $3
	AND origin = $4
	AND brand_id = $5
	AND category_id = $6
GROUP BY time, action, origin, brand_id, category_id
ORDER BY time"
                .replace("\t", "")
        );

        let req2 = AggregatesRequest {
            time_range: TimeRange {
                start: Utc::now(),
                end: Utc::now(),
            },
            action: Action::VIEW,
            origin: None,
            brand_id: None,
            category_id: None,
            aggregates: NAggregates(vec![Aggregates::COUNT, Aggregates::SUM_PRICE]),
        };

        let query2 = build_query(&req2);
        assert_eq!(
            query2.replace("\t", ""),
            "
SELECT time, action, SUM(count)::BIGINT, SUM(sum)::BIGINT
FROM aggregates
WHERE time >= $1 and time < $2
	AND action = $3
GROUP BY time, action
ORDER BY time"
                .replace("\t", "")
        );

        let req3 = AggregatesRequest {
            time_range: TimeRange {
                start: Utc::now(),
                end: Utc::now(),
            },
            action: Action::VIEW,
            origin: None,
            brand_id: Some("brand_id".to_string()),
            category_id: None,
            aggregates: NAggregates(vec![Aggregates::COUNT, Aggregates::SUM_PRICE]),
        };

        let query3 = build_query(&req3);
        assert_eq!(
            query3.replace("\t", ""),
            "
SELECT time, action, brand_id, SUM(count)::BIGINT, SUM(sum)::BIGINT
FROM aggregates
WHERE time >= $1 and time < $2
	AND action = $3
	AND brand_id = $4
GROUP BY time, action, brand_id
ORDER BY time"
                .replace("\t", "")
        );
    }

    #[tokio::test]
    async fn test_store() {
        env::set_var("DB_HOST", "st118vm101.rtb-lab.pl");
        env::set_var("DB_USER", "postgres");
        env::set_var("DB_PASSWORD", "postgres");
        env::set_var("DB_NAME", "allezone_analytics");
        env::set_var("DB_PORT", "5432");

        let store = PostgresStore::new().await;
        let req = AggregatesRequest {
            time_range: TimeRange {
                start: Utc::now(),
                end: Utc::now() + chrono::Duration::minutes(10),
            },
            action: Action::VIEW,
            origin: None,
            brand_id: Some("some_brand".to_string()),
            category_id: None,
            aggregates: NAggregates(vec![Aggregates::SUM_PRICE]),
        };
        let res = store.get_aggregates(&req).await;
        println!("{:?}", res);
    }

    #[test]
    fn test_date_format() {
        let dt = Utc.with_ymd_and_hms(2023, 3, 1, 0, 5, 0).unwrap();
        let dt_formatted = dt.naive_utc().format("%Y-%m-%dT%H:%M:%S").to_string();
        assert_eq!(dt_formatted, "2023-03-01T00:05:00");
    }
}
