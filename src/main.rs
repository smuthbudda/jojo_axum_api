#![allow(dead_code)]
use std::env;
use tower_http::cors::{Any, CorsLayer};
use axum::http::{header::CONTENT_TYPE, Method};

mod controllers;

// pub struct AppState {
//     db: MySqlPool,
// }

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");
    let _database_url = env::var("DATABASE_URL").expect("Database URL");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);
    
    let app = controllers::routes::create_router().layer(cors);
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
