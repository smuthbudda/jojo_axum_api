use crate::routes::routes::AppState;
use axum::http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use moka::future::Cache;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use std::{env, sync::Arc, thread, time::Duration};
use sysinfo::System;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

mod config;
mod routes;
mod db;
mod models;
mod req_models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");
    let config = config::Config::init();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let connection_pool: Pool<Postgres> = db::connect_to_database().await;
    let (tx, _) = broadcast::channel::<routes::routes::Snapshot>(1);
    let cache:Cache<Uuid, req_models::token::TokenDetails> = Cache::builder()
        .max_capacity(50_000)
        .time_to_live(Duration::from_secs(60 * 60 * 24))
        .time_to_idle(Duration::from_secs(60 * 60 * 24))
        .build();
    tracing_subscriber::fmt::init();

    sqlx::migrate!().run(&connection_pool).await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let app_state = Arc::new(AppState {
        db: connection_pool.clone(),
        env: config.clone(),
        tx: tx.clone(),
        cache: cache.clone(),
    });

    let app = routes::routes::create_router(app_state).layer(cors);

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);

            let ten_millis = std::time::Duration::new(3, 0);
            thread::sleep(ten_millis);
        }
    });

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("✅ API is being served! on {}", server_address);
    axum::serve(listener, app).await.unwrap();
}
