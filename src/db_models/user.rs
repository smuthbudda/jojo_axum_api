

#[derive(Default, Debug, Clone, PartialEq, sqlx::FromRow, Eq)]
pub struct User {
    id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: String,
    pub phone: Option<String>,
    password: String,
}

impl User {
    pub fn get_hash(&self) -> &String {
        &self.password
    }
}

impl axum_login::AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

