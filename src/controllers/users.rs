use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db_models::user::User, req_models::user_req::{CreateUserRequest, UserResponse}};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginRequest {
    user_name: String,
    password: String,
}

pub async fn get_users_handler(
    State(_pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let users = vec![
    //     CreateUserRequest::create_user(1, "John".to_string(), "Doe".to_string()),
    //     CreateUserRequest::create_user(2, "Jane".to_string(), "Doe".to_string()),
    // ];

    let json_response = serde_json::json!({
        "status": "ok",
    });

    Ok(Json(json_response))
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<crate::db_models::user::User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE user_name = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(&req.user_name)
    .fetch_optional(&pool)
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
    .execute(&pool)
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

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<crate::db_models::user::User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE user_name = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(&req.user_name)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if user.is_none() {
        return Ok(Json(serde_json::json!({"status":"No match found"})));
    }

    let user = user.unwrap();

    let is_match = bcrypt::verify(&req.password, &user.get_hash());

    match is_match {
        true => {
            let response = UserResponse::new(
                user.first_name,
                user.last_name,
                user.email,
                user.phone,
                user.user_name,
            );
            let json_response = serde_json::json!({
                "user": response
            });
            Ok(Json(serde_json::json!(json_response)))
        }
        false => Result::Ok(Json(serde_json::json!({"status":"Invalid Password"}))),
    }
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub async fn get_user_details(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>  {
    let user: Option<crate::db_models::user::User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE id = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if user.is_none() {
        return Ok(Json(serde_json::json!({"status":"great Successs"})));
    }

    Ok(Json(serde_json::json!({"status":"Not Found"})))
}
