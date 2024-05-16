use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use tracing::info;

#[derive(Debug, Clone, FromRow)]
pub struct User { 
    username: String,
    pw_hash: String,
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

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error)
}

#[derive(Clone, Debug)]
pub struct Backend {
    db: SqlitePool
}

impl Backend {
    /// Create a new backend with a `SqlitePool`.
    pub fn new (db: SqlitePool) -> Self {
        Self { db }
    }

    /// Returns the user if successfully registered, and `None` otherwise (ie, if user already exists).
    #[tracing::instrument]
    pub async fn register(&mut self, Credentials {username, password}: Credentials)
    -> Result<Option<User>, BackendError> 
    {
        if self.get_user(&username).await?.is_some() {
            Ok(None)
        }
        else {
            let user = User {username: username.clone(), pw_hash: password.clone() };
            sqlx::query("insert into users (username, pw_hash) values ($1, $2)")
                .bind(username)
                .bind(password)
                .execute(&self.db)
                .await?;
            Ok(Some(user))
        }   
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(&self, Credentials {username, password}: Self::Credentials) 
    -> Result<Option<Self::User>, Self::Error> 
    {   
        if let Some(user) = self.get_user(&username).await? {
            if password == user.pw_hash { // hash is actually 1:1 of password string
                info!("Successful login {:?}", username);
                return Ok(Some(user));
            }
            else {
                info!("Wrong password {:?}", username);
            }
        }
        else {
            info!("No such user {:?}", username);
        }
        Ok(None) 
    }

    async fn get_user(&self, username: &UserId<Self>) 
    -> Result<Option<Self::User>, Self::Error> 
    {
        info!("Finding user {:?}", username);

        sqlx::query_as("select * from users where username = $1 limit 1")
            .bind(username)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| { BackendError::Sqlx(e) })
    }
}