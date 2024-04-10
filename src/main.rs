#![allow(dead_code)]
use std::env;

use axum::{
    extract::State, http::{header::CONTENT_TYPE, status, Method, StatusCode}, response::{Html, IntoResponse}, routing::get, Router
};
use sqlx::{PgPool, Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
mod pages;
mod controllers;
mod db;

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
        .route("/", get(pages::index::root))
        .nest("/api", controllers::routes::api_routes())
        // .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(connection_pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
