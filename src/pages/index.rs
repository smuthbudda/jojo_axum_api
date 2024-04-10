use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {
    title: &'static str,
}

pub async fn root() -> impl IntoResponse {
    let root = RootTemplate { title: "title" };
    (StatusCode::OK, Html(root.render().unwrap()))
}
