use axum::{routing::get, Router};
use sqlx::PgPool;

use super::pointspage::index;

pub fn page_routes() -> Router<PgPool> {
    Router::new()
        .route("/points", get(index))
        .route("/points/:category/:gender/:event", get(index))
}
