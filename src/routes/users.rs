use axum::{
    extract::{Path, State}, http::{header, HeaderMap, Response, StatusCode}, response::IntoResponse, Extension, Json
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;


use crate::{db_models::user::User, req_models::{token::TokenDetails, user_req::CreateUserRequest}};

use super::{
    jwt_auth::JWTAuthMiddleware, routes::AppState, utils::token
};

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
    let refresh_token_details = generate_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    )?;

    cache_token(&data, &access_token_details).await;
    cache_token(&data, &refresh_token_details).await;

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build((
        "refresh_token",
        refresh_token_details.token.unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);

    Ok(response)
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

async fn cache_token(data: &Arc<AppState>, token: &TokenDetails) {
    data.cache.insert(token.token_uuid, token.clone()).await;
}
