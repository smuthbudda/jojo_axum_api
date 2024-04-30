#![allow(dead_code)]
use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use crate::controllers::users;

pub fn api_routes() -> Router<PgPool> {
    let user_routes = Router::new()
        .route("/", get(users::get_users_handler))
        .route("/:id", get(users::get_user_details))
        .route("/", post(users::create_user_handler))
        .route("/login", post(users::login_handler));

    let health_check_routes = Router::new()
        .route("/check", get(super::health_check::health_check));

    let points_routes = Router::new()
        .route("/read", get(super::iaaf_points::read_iaaf_json))
        .route("/points/:category/:gender/:event", get(super::iaaf_points::get_value));

    Router::new()
        .nest("/user", user_routes)
        .nest("/health-check", health_check_routes)
        .nest("/world-aths", points_routes)
}



