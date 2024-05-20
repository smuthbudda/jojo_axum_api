#![allow(dead_code)]

use std::sync::Arc;

use axum::{extract::State, response::Html};

use super::routes::AppState;

pub async fn health_check(State(_data): State<Arc<AppState>>) -> Html<&'static str> {
    Html("The API is up and running!")
}
