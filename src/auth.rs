use std::{collections::HashMap, convert::Infallible};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use axum::http::StatusCode;

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    pw_hash: Vec<u8>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(Clone, Default)]
pub struct Backend {
    users: HashMap<i64, User>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    user_id: i64,
    password: String
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Infallible;

    async fn authenticate(&self, Credentials { user_id, password }: Self::Credentials) 
    -> Result<Option<Self::User>, Self::Error> 
    {   
        if let Some(user) = self.users.get(&user_id).cloned() {
            if password.into() == user.pw_hash {
                return Ok(Some(user));
            }
        }
        Err(Self::Error)
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
    }
}