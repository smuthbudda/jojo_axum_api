#![allow(dead_code)]
use std::env;

use axum::{
    http::{header::CONTENT_TYPE, Method}, Router
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
mod pages;
mod controllers;
mod db;
mod db_models;
mod req_models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address: String =
        env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    let connection_pool: Pool<Postgres> = db::connect_to_database().await;
    
    sqlx::migrate!().run(&connection_pool).await.unwrap();
    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .nest("/pages", pages::pagerouter::page_routes())
        .nest("/api", controllers::routes::api_routes())
        .with_state(connection_pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    println!("✅ API is being served!");
    axum::serve(listener, app).await.unwrap();
}