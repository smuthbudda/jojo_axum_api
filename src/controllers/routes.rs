#![allow(dead_code)]
use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::controllers::users::get_users_handler;


pub fn api_routes() -> Router<PgPool> {
    let user_routes = Router::new()
        .route("/all", get(get_users_handler));

    Router::new()
        .nest("/user", user_routes)
}



