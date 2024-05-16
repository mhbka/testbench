use axum_login::AuthUser;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User { 
    pub username: String,
    pub pw_hash: String,
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.username.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.pw_hash.as_bytes()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String
}