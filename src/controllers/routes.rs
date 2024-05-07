#![allow(dead_code)]
use std::sync::Arc;

use crate::controllers::users;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: crate::config::Config,
    pub redis_client: redis::Client,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let user_routes = Router::new()
        .route("/", get(users::get_users_handler))
        .route("/:id", get(users::get_user_details_handler))
        .route("/", post(users::create_user_handler))
        .route("/login", post(users::login_handler));

    let health_check_routes = Router::new().route("/check", get(super::health_check::health_check));

    let points_routes = Router::new()
        .route("/read", get(super::iaaf_points::read_iaaf_json))
        .route(
            "/points/:category/:gender/:event",
            get(super::iaaf_points::get_value),
        );
    let router = Router::new()
        .nest("/user", user_routes)
        .nest("/health-check", health_check_routes)
        .nest("/world-aths", points_routes)
        .with_state(app_state);

    Router::new().nest("/api", router)
}
