mod routes;

use axum::Extension;
use axum::routing::get;
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use dotenv::dotenv;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::env;

use axum::{routing::post, Router};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use routes::*;

fn main() {
    dotenv().ok();

    tokio::runtime::Builder::new_multi_thread()
        // .worker_threads(4)
        .enable_all()
        .max_blocking_threads(1024)
        .build()
        .unwrap()
        .block_on(async {
            let json_output = env::var("JSON_LOGGING").unwrap_or("false".to_string()) == "true";
            init_loggin(json_output);

            let ip = "0.0.0.0";
            let port = env::var("GATEWAY_PORT").expect("GATEWAY_PORT must be set");

			let prom_handle = PrometheusBuilder::new()
				.install_recorder()
				.expect("failed to install Prometheus recorder");				

            let app = Router::new()
                .route("/user_tags", post(user_tags))
                .route("/user_profiles/:cookie", post(user_profiles))
                .route("/aggregates", post(aggregates))
				.route("/metrics", get(metrics))
				.layer(Extension(prom_handle))
				.layer(opentelemetry_tracing_layer())
                .layer(Extension(SharedStore::new().await));

            axum::Server::bind(&format!("{}:{}", ip, port).parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        })
}

fn init_loggin(json_output: bool) {
    let mut layers = Vec::new();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true);
    
    let fmt_layer = match json_output {
        true => fmt_layer.json().flatten_event(true).boxed(),
        false => fmt_layer.boxed(),
    };
    layers.push(fmt_layer);

    let env_filter =
        EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layers)
        .init()
}
