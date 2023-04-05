mod routes;

use std::env;
use dotenv::dotenv;

use axum::{Router, routing::post};
use tower_http::trace::{TraceLayer, Trace};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use routes::*;

fn main() {
    dotenv().ok();

    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        let json_output = env::var("JSON_LOGGING").unwrap_or("false".to_string()) == "true";
        init_loggin(json_output);

        let ip = env::var("SERVER_IP").expect("SERVER_IP must be set");
        let port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");

        let app = Router::new()
            .route("/user_tags", post(user_tags))
            .route("/user_profiles/:cookie", post(user_profiles))
            .route("/aggregates", post(aggregates));

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