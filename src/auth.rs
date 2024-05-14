use std::{collections::HashMap, convert::Infallible, sync::{Arc, Mutex}};
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub user_id: i64,
    pub password: String
}

#[derive(Clone)]
pub struct Backend {
    users: Arc<Mutex<HashMap<i64, User>>>
}

impl Backend {
    pub fn new (users: Arc<Mutex<HashMap<i64, User>>>) -> Self {
        Self { users }
    }

    /// Returns the user if successfully registered, and `None` otherwise (ie, if user already exists).
    pub fn register(&mut self, Credentials {user_id, password}: Credentials)
    -> Result<Option<User>, Infallible> 
    {
        println!("{user_id}");
        if self.users.lock().unwrap().get(&user_id).is_some() {
            Ok(None)
        }
        else {
            let user = User {id: user_id, pw_hash: password.into_bytes() };
            self.users.lock().unwrap().insert(user_id, user.clone());
            println!("{:?}", self.users);
            Ok(Some(user))
        }   
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Infallible;

    async fn authenticate(&self, Credentials {user_id, password}: Self::Credentials) 
    -> Result<Option<Self::User>, Self::Error> 
    {   
        println!("{:?}", self.users);
        if let Some(user) = self.users.lock().unwrap().get(&user_id).cloned() {
            if password.as_bytes() == user.pw_hash { // hash is actually 1:1 of password string
                return Ok(Some(user));
            }
            else {
                println!("bad password");
            }
        }
        Ok(None) // user doesn't exist or password is wrong
    }

    async fn get_user(&self, user_id: &UserId<Self>) 
    -> Result<Option<Self::User>, Self::Error> 
    {
        Ok(self.users.lock().unwrap().get(user_id).cloned())
    }
}