use std::env;
use axum::{
    routing::get,
    Router,
};

mod controllers;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");
    let _database_url = env::var("DATABASE_URL").expect("Database URL");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    // build our application with a single route
    let app = controllers::routes::get_routes();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

