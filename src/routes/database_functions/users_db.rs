use sqlx::{Pool, Postgres};

use crate::{models::user::User, req_models::user_req::CreateUserRequest};

pub async fn create_user(pool : &Pool<Postgres>, dto : CreateUserRequest, hash: String) -> bool{
    let insert_result = sqlx::query(
        r#"INSERT INTO users (user_name, first_name, last_name, email, phone, active, password) 
        VALUES ($1, $2, $3, $4, $5, TRUE, $6);
        "#,
    )
    .bind(dto.user_name)
    .bind(dto.first_name)
    .bind(dto.last_name)
    .bind(dto.email)
    .bind(dto.phone)
    .bind(hash)
    .execute(pool)
    .await;

    match insert_result{
        Err(_e) => {
            return false
        },
        Ok(result) => {
           result.rows_affected() == 1 
        }
    }
}

pub async fn get_user_by_username(pool : &Pool<Postgres>, user_name : &String) -> Option<User>{
   let user : Option<User> =  sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE user_name = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(user_name)
    .fetch_optional(pool)
    .await.expect("Error loading user.");
    user
}
