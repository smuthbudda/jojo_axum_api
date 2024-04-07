use std::time::Duration;

use sea_orm::{ConnectOptions, Database};

fn main() {
    println!("Hello, world!");

    
}


async fn database_connection(){
    let mut opt = ConnectOptions::new("postgres://postgres:145269@192.168.1.114:5432/rust_seaorm");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;
}