pub mod auth;
pub mod routes;
pub mod db;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let pool = db::get_pool().await;

    let tracer = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracer).unwrap();

    let app = Router::new()
        .merge(routes::routes(pool))
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}