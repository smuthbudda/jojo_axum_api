use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    user_id: u32,
    first_name: String,
    last_name: String,
    birth_date: NaiveDate,
}

impl User {
    pub fn create_user(user_id: u32, first_name: String, last_name: String, birth_date: NaiveDate) -> Self {
        User {user_id,first_name,last_name,birth_date}
    }
}
