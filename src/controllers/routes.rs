#![allow(dead_code)]
use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::controllers::users;

use super::{health_check::health_check, iaaf_points::read_iaaf_json};


pub fn api_routes() -> Router<PgPool> {
    let user_routes = Router::new()
        .route("/all", get(users::get_users_handler));

    let health_check_routes = Router::new()
        .route("/check", get(health_check));

    let points_routes = Router::new().route("/read", get(read_iaaf_json));

    Router::new()
        .nest("/user", user_routes)
        .nest("/health-check", health_check_routes)
        .nest("/world-aths", points_routes)
}



