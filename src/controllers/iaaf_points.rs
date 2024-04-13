use axum::response::Html;

pub async fn read_iaaf_json(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
) -> Html<&'static str> {
    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(id) FROM points"#)
        .fetch_one(&pool)
        .await
        .unwrap();

    if count > 200000 {
        return Html("Records already exist.");
    }

    let models = crate::db_models::iaaf_points::read_from_file();
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
