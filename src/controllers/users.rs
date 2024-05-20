use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::{extract::cookie::{Cookie, SameSite}, headers};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    db_models::user::User,
    req_models::user_req::CreateUserRequest,
};

use super::{routes::AppState, token::{self, TokenDetails}};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginRequest {
    user_name: String,
    password: String,
}

pub async fn get_users_handler(
    State(_data): State<Arc<AppState>>,
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

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<crate::db_models::user::User> = sqlx::query_as(
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



    if user.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status":"Error user not found."})),
        ));
    }

    let user = user.unwrap();

    let is_match = bcrypt::verify(&req.password, &user.get_hash());

    if !is_match {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let access_token_details = generate_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );

    
    Ok(response)
}

pub async fn get_user_details_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<crate::db_models::user::User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE id = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(id)
    .fetch_optional(&data.db)
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

fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

fn generate_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, (StatusCode, Json<serde_json::Value>)> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error generating token: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}

