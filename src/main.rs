use axum::Router;


mod auth;
mod db;
mod config;

#[tokio::main]
async fn main() {
    let pool = db::init_pool().await;

    let tracer = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracer).unwrap();

    let app = Router::new()
        .merge(auth::router(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}