#![allow(dead_code)]
use std::sync::Arc;

use super::{
    iaaf_points::{get_value, read_iaaf_json},
    system_info::{get_system_details_handler, realtime_cpu_handler},
    users::{create_user_handler, get_user_details_handler, get_users_handler, login_handler},
};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;

pub type Snapshot = Vec<f32>;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: crate::config::Config,
    pub tx: broadcast::Sender<Snapshot>,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let user_routes = Router::new()
        .route("/", get(get_users_handler))
        .route("/:id", get(get_user_details_handler))
        .route("/", post(create_user_handler))
        .route("/login", post(login_handler));

    let health_check_routes = Router::new().route("/check", get(super::health_check::health_check));

    let points_routes = Router::new()
        .route("/read", get(read_iaaf_json))
        .route("/points/:category/:gender/:event", get(get_value));

    let system_routes = Router::new()
        .route("/cpu", get(realtime_cpu_handler))//web socket 
        .route("/details", get(get_system_details_handler));

    let router = Router::new()
        .nest("/user", user_routes)
        .nest("/health-check", health_check_routes)
        .nest("/world-aths", points_routes)
        .nest("/system", system_routes)
        .with_state(app_state);

    Router::new().nest("/api", router)
}
