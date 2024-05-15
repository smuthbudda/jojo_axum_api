#![allow(dead_code)]
use crate::controllers::routes::AppState;
use axum::{error_handling::HandleErrorLayer, http::{header::CONTENT_TYPE, Method}, middleware::map_request};
use sqlx::{Pool, Postgres};
use std::{env, sync::Arc};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
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
    let (tx, _) = broadcast::channel::<controllers::routes::Snapshot>(1);

    tracing_subscriber::fmt::init();

    sqlx::migrate!().run(&connection_pool).await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app_state = Arc::new(AppState {
        db: connection_pool.clone(),
        env: config.clone(),
        tx: tx.clone(),
    });

    let app = controllers::routes::create_router(app_state).layer(cors);

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("✅ API is being served! on {}", server_address);
    axum::serve(listener, app).await.unwrap();
}
