use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse}
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}

pub async fn root(axum::extract::State(_pool): axum::extract::State<PgPool>) -> impl IntoResponse {
    let index = IndexTemplate { title: "title" };
    // sqlx::query_as!(IndexTemplate)
    (StatusCode::OK, Html(index.render().unwrap()))
}
