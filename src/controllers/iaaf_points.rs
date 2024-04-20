use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use crate::db_models::iaaf_points::{Category, Gender, Points, PointsSearchQueryParams};
use sqlx::PgPool;

static FILE_LOCATION: &str = "data/WorldAthletics.json";

pub async fn read_iaaf_json(State(pool): State<PgPool>) -> Html<&'static str> {
    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(id) FROM points"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    if count > 200000 {
        return Html("Records already exist.");
    }

    let models = read_from_file();
    print!("inserting into database");
    let html: &str = "Success";
    for points_model in models {
        let query_result = sqlx::query(
            r#"INSERT INTO points (points, gender, category, event, mark)
            VALUES
            ($1, $2, $3, $4, $5)"#,
        )
        .bind(points_model.points)
        .bind(&points_model.gender)
        .bind(&points_model.category)
        .bind(&points_model.event)
        .bind(points_model.mark)
        .execute(&pool)
        .await;

        match query_result {
            Ok(_) => {
                println!("Insert successful!");
                // html = "Insert success"
            }
            Err(err) => {
                println!("Error: {}", err);
                // html = "Insert fail"
            }
        }
    }

    Html(html)
}

pub async fn get_value(
    Path((category, gender, event)): Path<(Category, Gender, String)>,
    Query(params): Query<PointsSearchQueryParams>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if (params.mark.is_some() && params.points.is_some()) || (params.mark.is_none() && params.points.is_none()) {
        let bad_json = serde_json::json!({
            "status": "Bad Request"
        });
        return Err((StatusCode::NOT_FOUND, Json(bad_json)));
    }
    
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
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "status": "ok",
        "count": query_result.len(),
        "points": query_result
    });

    Ok(Json(json_response))
}

fn read_from_file() -> Vec<Points> {
    println!("Reading json file.");
    let file = std::fs::File::open(FILE_LOCATION).expect("Could not open file");
    let points: Vec<Points> = serde_json::from_reader(file).expect("error reading from file");

    return points;
}
