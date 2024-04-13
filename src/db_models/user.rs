use sqlx::types::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    contact_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    phone: Option<String>,
    password: String,
}

impl User {
    pub fn new(contact_id: sqlx::types::Uuid, first_name: String, last_name: String, email: String, phone: Option<String>, password: String) -> Self {
        Self {
            contact_id,
            first_name,
            last_name,
            email,
            phone,
            password,
        }
    }
}