use axum::{Json, Router, response::IntoResponse, http::StatusCode, routing::get};
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    user_id: u32,
    first_name: String,
    last_name: String,
    birth_date: NaiveDate,
}

impl User {
    pub fn create_user(user_id: u32, first_name: String, last_name: String, birth_date: NaiveDate) -> Self {
        User {user_id,first_name,last_name,birth_date}
    }
}


pub async fn get_users() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let users = vec![
        User::create_user(
            1,
            "John".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd(1990, 1, 1),
        ),
        User::create_user(
            2,
            "Jane".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd(1995, 5, 15),
        ),
    ];

    let single_user = users.first();
    let json_response = serde_json::json!({
        "status": "ok",
        "count": users.len(),
        "users": users
    });

    Ok(Json(json_response))
}

pub fn user_routes() -> Router {
    Router::new().route("/", get(get_users))
}