use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_derive::{Deserialize, Serialize};
use sqlx::PgPool;


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    user_id: u32,
    first_name: String,
    last_name: String,
}

impl User {
    fn create_user(user_id: u32, first_name: String, last_name: String) -> Self {
        User {user_id,first_name,last_name,}
    }
}


pub async fn get_users_handler(axum::extract::State(_pool): axum::extract::State<PgPool>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let users = vec![
        User::create_user(
            1,
            "John".to_string(),
            "Doe".to_string(),
        ),
        User::create_user(
            2,
            "Jane".to_string(),
            "Doe".to_string(),
        ),
    ];

    let json_response = serde_json::json!({
        "status": "ok",
        "count": users.len(),
        "users": users
    });

    Ok(Json(json_response))
<<<<<<< HEAD
=======
}

pub fn user_routes() -> Router {
    Router::new().route("/list", get(get_users_handler))
>>>>>>> a8456ca8f3046189e48ba02e849a2a06e08b6ab3
}