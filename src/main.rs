#![allow(dead_code)]
use std::{env, sync::Arc};
use axum::http::{header::CONTENT_TYPE, Method};
use redis::Client;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use crate::controllers::routes::AppState;
mod config;
mod controllers;
mod db;
mod db_models;
mod req_models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");
    let config = config::Config::init();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    let connection_pool: Pool<Postgres> = db::connect_to_database().await;

    sqlx::migrate!().run(&connection_pool).await.unwrap();

    let redis_client = match Client::open(config.redis_url.to_owned()) {
        Ok(client) => {
            println!("✅Connection to the redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app_state = Arc::new(AppState {
        db: connection_pool.clone(),
        env: config.clone(),
        redis_client: redis_client.clone(),
    });

    let app = controllers::routes::create_router(app_state).layer(cors);

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("✅ API is being served! on {}", server_address);
    axum::serve(listener, app).await.unwrap();
}
