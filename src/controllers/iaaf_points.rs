use axum::{
    extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json
};
use serde_json;
use crate::db_models::iaaf_points::{Category, Gender, PointsInsert, PointsSearchQueryParams};
use sqlx::PgPool;
use std::{error::Error, sync::Arc};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

use super::routes::AppState;

static FILE_LOCATION: &str = "data/WorldAthletics.json";

pub async fn read_iaaf_json(State(data): State<Arc<AppState>>) -> Result<impl IntoResponse, (axum::http::StatusCode, Json<serde_json::Value>)>{
    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(id) FROM points"#)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let mut json_response = serde_json::json!({
        "Message": "Points Already Exists"
    });
    
    if count > 200000 {      
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let models = read_file_async().await;

    match models {
        Err(e) => {
            let error = "Error: ".to_string() + &e.to_string();
            json_response = serde_json::json!({
                "Message": error
            });
            return Ok(Json(json_response));
        },
        Ok(_) => {
            print!("inserting into database");
            let mut count = 0;
            for points_model in models.unwrap(){
                let query_result = sqlx::query(
                    r#"INSERT INTO points (points, gender, category, event, mark)
                            VALUES ($1, $2, $3, $4, $5)"#,
                )
                .bind(points_model.points)
                .bind(&points_model.gender)
                .bind(&points_model.category)
                .bind(&points_model.event)
                .bind(points_model.mark)
                .execute(&data.db)
                .await;

                match query_result {
                    Ok(_) => {
                        println!("Insert successful!");
                        count += 1;
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        // html = "Insert fail"
                    }
                }
            }
            
            json_response = serde_json::json!({
                "Status" : "Values Added!",
                "Count" : count
            });

            return Ok(Json(json_response));
        }
    }
}

pub async fn get_value(
    Path((category, gender, event)): Path<(Category, Gender, String)>,
    Query(params): Query<PointsSearchQueryParams>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if (params.mark.is_some() && params.points.is_some())
        || (params.mark.is_none() && params.points.is_none())
    {
        let bad_json = serde_json::json!({
            "status": "Bad Request"
        });
        return Err((StatusCode::NOT_FOUND, Json(bad_json)));
    }
    
    let query_result = sqlx::query_as::<_, PointsInsert>(
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
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "points": query_result
    });

    Ok(Json(json_response))
}

// fn read_from_file() -> Vec<Points> {
//     println!("Reading json file.");
//     let file = std::fs::File::open(FILE_LOCATION).expect("Could not open file");
//     let points: Vec<Points> = serde_json::from_reader(file).expect("error reading from file");

//     return points;
// }

async fn read_file_async() -> Result<Vec<PointsInsert>, Box<dyn Error>> {
    println!("Reading json file.");
    let file = File::open(FILE_LOCATION).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;

    let points: Vec<PointsInsert> = serde_json::from_slice(&buffer)?;

    Ok(points)
}
