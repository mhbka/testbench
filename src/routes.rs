use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::{
    http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router
};
use sqlx::SqlitePool;
use crate::auth::{
    Backend,
    Credentials
};
use axum_login::{
    login_required, 
    tower_sessions::{MemoryStore, SessionManagerLayer}, 
    AuthManagerLayer, 
    AuthManagerLayerBuilder, AuthnBackend
};
use axum_macros::debug_handler;

type AuthSession = axum_login::AuthSession<Backend>;

pub fn routes(pool: SqlitePool) -> Router {
    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let backend = Backend::new(pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    Router::new()
        .route("/protected", get(|| async { "helo" }))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(auth_layer)
}

#[debug_handler]
async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/protected").into_response()
}

#[debug_handler]
async fn register(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    match auth_session.backend.register(creds).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_err() {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            return Redirect::to("/protected").into_response();
        },
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }    
}