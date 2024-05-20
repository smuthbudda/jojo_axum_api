use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
}

// impl CreateUserRequest {
//     pub fn new(
//         first_name: String,
//         last_name: String,
//         email: String,
//         phone: Option<String>,
//         password: String,
//         user_name: String,
//     ) -> CreateUserRequest {
//         CreateUserRequest {
//             first_name: first_name,
//             last_name: last_name,
//             email: email,
//             phone: phone,
//             password: password,
//             user_name: user_name,
//         }
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserResponse{
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: String,
    pub phone: Option<String>,
}

// impl UserResponse {
//     pub fn new(
//         first_name: String,
//         last_name: String,
//         email: String,
//         phone: Option<String>,
//         user_name: String,
//     ) -> UserResponse {
//         UserResponse {
//             first_name: first_name,
//             last_name: last_name,
//             email: email,
//             phone: phone,
//             user_name: user_name,
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}