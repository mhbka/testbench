mod backend;
mod handlers;
mod types;


use axum::{routing::{get, post}, Router};
use axum_login::{login_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayerBuilder};
use sqlx::SqlitePool;
use self::backend::Backend;
use self::handlers::{
    login,
    logout,
    register,
    whoami
};


pub fn router(pool: SqlitePool) -> Router 
{
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    Router::new()
        .route("/protected", get(|| async { "helo" }))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", get(logout))
        .route("/whoami", get(whoami))
        .layer(auth_layer)
}