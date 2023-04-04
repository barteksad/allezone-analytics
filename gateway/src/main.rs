mod routes;

use std::env;
use dotenv::dotenv;

use axum::{Router, routing::post};

use routes::*;

fn main() {
    dotenv().ok();

    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
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
