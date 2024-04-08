use axum::{routing::{get, Route}, Router};

use crate::controllers::users;

pub fn get_routes() -> Router{
    Router::new().nest("user", users::user_routes())   
}