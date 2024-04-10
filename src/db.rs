use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect_to_database() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    pool
}

pub fn map_db_err(err: sqlx::Error) -> (axum::http::StatusCode, String) {
    tracing::error!("{}", err);
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}