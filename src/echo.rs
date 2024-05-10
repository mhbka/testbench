use axum::routing::post;
use axum::{Json, Router};
use axum::response::IntoResponse;

pub fn routes() -> Router {
    Router::new().route("/echo", post(echo))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Echo {
    message: String
}

async fn echo(Json(echo): Json<Echo>) -> impl IntoResponse {
    Json(echo)
}