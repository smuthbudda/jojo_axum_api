#![allow(dead_code)]
use crate::db_models::iaaf_points::{Category, Gender, Points, PointsSearchQueryParams};
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "pointsindex.html")]
struct IndexTemplate {
    points_list: Vec<Points>,
}

pub async fn index(axum::extract::State(_pool): axum::extract::State<PgPool>) -> impl IntoResponse {
    let index = IndexTemplate {
        points_list: Vec::new(),
    };
    // sqlx::query_as!(IndexTemplate)
    (StatusCode::OK, Html(index.render().unwrap()))
}

#[derive(Template)]
#[template(path = "components/point.html")]
struct PointsTemplate {
    id: String,
    points: i32,
    gender: String,
    category: String,
    event: String,
    mark: f64,
}

pub async fn get_points(
    Path((category, gender, event)): Path<(Category, Gender, String)>,
    Query(params): Query<PointsSearchQueryParams>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let query_result = sqlx::query_as::<_, Points>(
        r#"
    SELECT * FROM points 
    WHERE 
        LOWER(category) = LOWER($1) AND 
        LOWER(gender) = LOWER($2) AND 
        LOWER(event) = LOWER($3) AND
        (ROUND(mark::numeric, 2) = $4 OR points = $5)
    ORDER BY CASE 
        WHEN mark IS NULL THEN 1 
        ELSE ABS($4 - 1400) 
    END
    FETCH FIRST 1 ROWS ONLY;"#,
    )
    .bind(category.to_string())
    .bind(gender.to_string())
    .bind(event)
    .bind(params.mark)
    .bind(params.points)
    .fetch_one(&pool)
    .await;

    let point_result = query_result.unwrap();
    let template: PointsTemplate = PointsTemplate {
        id: "123".to_owned(),
        points: point_result.points,
        gender: point_result.gender,
        category: point_result.category,
        event: point_result.event,
        mark: point_result.mark,
    };
    (StatusCode::OK, Html(template.render().unwrap()))
}
