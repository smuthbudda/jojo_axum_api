use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{models::user::User, req_models::user_req::CreateUserRequest};

use super::{jwt_auth::JWTAuthMiddleware, routes::AppState};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginRequest {
    user_name: String,
    password: String,
}

pub async fn get_users_handler(
    State(_data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status": "ok",
    });

    Ok(Json(json_response))
}

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE user_name = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(&req.user_name)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    match user {
        None => println!("user does not exist"),
        Some(_user) => return Ok(Json(serde_json::json!({"status":"User Already Exists"}))),
    }

    let hash = hash_password(&req.password);
    let insert_result = sqlx::query(
        r#"INSERT INTO users (user_name, first_name, last_name, email, phone, active, password) 
        VALUES ($1, $2, $3, $4, $5, TRUE, $6);
        "#,
    )
    .bind(req.user_name)
    .bind(req.first_name)
    .bind(req.last_name)
    .bind(req.email)
    .bind(req.phone)
    .bind(hash)
    .execute(&data.db)
    .await;

    match insert_result {
        Ok(_) => Ok(Json(
            serde_json::json!({"status":"OK", "Message":"User Created"}),
        )),
        Err(e) => {
            println!("{}", e);
            Ok(Json(serde_json::json!({"status":"Error inserting"})))
        }
    }
}

pub async fn get_user_details_handler(
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": &jwtauth.user
        })
    });

    Ok(Json(json_response))
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}
