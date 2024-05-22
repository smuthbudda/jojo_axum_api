use super::{routes::AppState, utils::token};
use crate::models::user::User;
use crate::req_models::token::TokenDetails;
use axum::{
    extract::State,
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginRequest {
    user_name: String,
    password: String,
}

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<crate::models::user::User> = sqlx::query_as(
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

    cache_jwt_token(&data, &access_token_details).await;
    cache_jwt_token(&data, &refresh_token_details).await;

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

pub async fn refresh_access_token_handler(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let message = "could not refresh access token";

    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": message
            });
            (StatusCode::FORBIDDEN, Json(error_response))
        })?;

    let refresh_token_details =
        match token::verify_jwt_token(data.env.refresh_token_public_key.to_owned(), &refresh_token)
        {
            Ok(token_details) => token_details,
            Err(e) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format_args!("{:?}", e)
                });
                return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
            }
        };

    let cache = &data.cache;
    let cache_token  = cache.get(&refresh_token_details.token_uuid).await.unwrap();
    
    let user: Option<User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE id = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(cache_token.user_id)
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

    let access_token_details: TokenDetails = generate_token(
        user.unwrap().id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    cache_jwt_token(&data, &access_token_details).await;

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
        logged_in_cookie.to_string().parse().unwrap(),
    );
    response.headers_mut().extend(headers);
    Ok(response)
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

async fn cache_jwt_token(data: &Arc<AppState>, token: &TokenDetails) {
    data.cache.insert(token.token_uuid, token.clone()).await;
}
