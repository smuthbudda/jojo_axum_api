// use std::sync::Arc;
// use axum::{
//     extract::State,
//     http::{header, Request, StatusCode},
//     middleware::Next,
//     response::IntoResponse,
//     Json, body::Body,
// };
// use axum_extra::extract::cookie::CookieJar;
// use serde::{Deserialize, Serialize};
// use crate::db_models::user::User;

// #[derive(Debug, Serialize)]
// pub struct ErrorResponse{
//     pub status: &'static str,
//     pub message: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct JWTAuthMiddleware {
//     pub user: User,
//     pub access_token_uuid: uuid::Uuid,
// }

