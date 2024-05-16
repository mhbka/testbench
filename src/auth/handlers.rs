use axum::{
    Json,
    Form,
    http::StatusCode, 
    response::{IntoResponse, Redirect} 
};
use tracing::info;
use axum_macros::debug_handler; // useful if a handler has a weird error
use super::types::{Credentials, User};
use super::backend::Backend;

type AuthSession = axum_login::AuthSession<Backend>;

#[tracing::instrument(skip_all)]
pub async fn login(
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

#[tracing::instrument(skip_all)]
pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(Some(_)) => Redirect::to("/").into_response(),
        Ok(None) => StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[tracing::instrument(skip_all)]
pub async fn register(
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

#[tracing::instrument(skip_all)]
#[debug_handler]
pub async fn whoami(auth_session: AuthSession) -> Result<Json<User>, StatusCode> {
    match auth_session.user {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::UNAUTHORIZED)
    }
}
