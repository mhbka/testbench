use std::{collections::HashMap, convert::Infallible, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone)]
pub struct User {
    username: String,
    pw_hash: String,
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.username
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String
}

#[derive(Clone)]
pub struct Backend {
    db: SqlitePool
}

impl Backend {
    /// Create a new backend with a `SqlitePool`.
    pub fn new (db: SqlitePool) -> Self {
        Self { db }
    }

    /// Returns the user if successfully registered, and `None` otherwise (ie, if user already exists).
    pub async fn register(&mut self, Credentials {username, password}: Credentials)
    -> Result<Option<User>, Infallible> 
    {

        if self.get_user(&user_id).await?.is_some() {
            Ok(None)
        }
        else {
            let user = User {username, pw_hash: password };
            sqlx::query(
                r#"insert into "user" (username, pw_hash) values ($1, $2) returning username"#,
                username, password
            )
            Ok(Some(user))
        }   
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Infallible;

    async fn authenticate(&self, Credentials {username, password}: Self::Credentials) 
    -> Result<Option<Self::User>, Self::Error> 
    {   
        if let Some(user) = self.get_user(&username).await? {
            if password == user.pw_hash { // hash is actually 1:1 of password string
                return Ok(Some(user));
            }
            else {
                println!("bad password");
            }
        }
        Ok(None) // user doesn't exist or password is wrong
    }

    async fn get_user(&self, username: &UserId<Self>) 
    -> Result<Option<Self::User>, Self::Error> 
    {
        sqlx::query_as!(
            Credentials,
            r#"select * from "user" where username = $1"#,
            username
        )
            .fetch_optional(self.db)
            .await?
    }
}