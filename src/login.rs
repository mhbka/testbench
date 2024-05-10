use axum::{
    http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router
};
use crate::auth::{
    Backend,
    Credentials
};


pub fn login_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/protected", get(|| async {"Congratz!"}))
}



type AuthSession = axum_login::AuthSession<Backend>;

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