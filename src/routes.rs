use axum::{
    http::StatusCode, 
    response::{IntoResponse, Redirect}, 
    routing::{get, post}, 
    Form, Router
};
use axum_login::{
    login_required, 
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder
};
use crate::auth::{
    Backend,
    Credentials
};
use tracing::info;
use axum_macros::debug_handler;
use sqlx::SqlitePool;

type AuthSession = axum_login::AuthSession<Backend>;

pub fn routes(pool: SqlitePool) -> Router {
    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let backend = Backend::new(pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // Return the router
    Router::new()
        .route("/protected", get(|| async { "helo" }))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(auth_layer)
}

#[debug_handler]
#[tracing::instrument(skip_all)]
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
#[tracing::instrument(skip_all)]
async fn register(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    match auth_session.backend.register(creds).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_err() {
                info!("User registered successfully but error occurred");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            info!("User registered successfully");
            return Redirect::to("/protected").into_response();
        },
        Ok(None) => {
            info!("User already exists");
            return StatusCode::CONFLICT.into_response()
        },
        Err(err) => {
            info!("Error occurred: {err:?}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }    
}