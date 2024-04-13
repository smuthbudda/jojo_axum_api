#![allow(dead_code)]

use axum::response::Html;

pub async fn health_check(axum::extract::State(_pool): axum::extract::State<sqlx::PgPool>) -> Html<&'static str> {
    Html("<h1>The API is up and running!</h1>")
}
