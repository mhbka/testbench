pub mod auth;
pub mod echo;
pub mod login;

use axum::{
    routing::{post, get},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .merge(echo::routes())
    .merge(login::routes())
    .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}