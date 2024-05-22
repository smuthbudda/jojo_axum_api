#![allow(dead_code)]
use std::sync::Arc;

use crate::req_models::token::TokenDetails;

use super::{
    auth::{login_handler, refresh_access_token_handler},
    iaaf_points::{get_value, read_iaaf_json},
    jwt_auth::auth,
    system_info::{get_system_details_handler, realtime_cpu_handler},
    users::{create_user_handler, get_user_details_handler, get_users_handler},
};

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use moka::future::Cache;
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;
use uuid::Uuid;

pub type Snapshot = Vec<f32>;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: crate::config::Config,
    pub tx: broadcast::Sender<Snapshot>,
    pub cache: Cache<Uuid, TokenDetails>,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let user_routes = Router::new()
        .route("/", get(get_users_handler))
        .route("/", post(create_user_handler))
        .route(
            "/me",
            get(get_user_details_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        );

    let health_check_routes = Router::new().route("/check", get(super::health_check::health_check));

    let points_routes = Router::new()
        .route("/read", get(read_iaaf_json))
        .route("/points/:category/:gender/:event", get(get_value));

    let system_routes = Router::new()
        .route("/cpu", get(realtime_cpu_handler)) //web socket
        .route("/details", get(get_system_details_handler));
    
    let auth_routes = Router::new()
    .route("/refresh_token", get(refresh_access_token_handler))
    .route("/login", post(login_handler));

    let router = Router::new()
        .nest("/user", user_routes)
        .nest("/health-check", health_check_routes)
        .nest("/world-aths", points_routes)
        .nest("/system", system_routes)
        .nest("/auth", auth_routes)
        .with_state(app_state);
    
    Router::new().nest("/api", router)
}
