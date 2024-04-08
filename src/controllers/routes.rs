#![allow(dead_code)]
use axum::Router;

use crate::controllers::users::user_routes;


pub fn create_router() -> Router{
    Router::new().nest("/users", user_routes())
}


