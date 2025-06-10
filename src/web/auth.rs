use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

use crate::users::{AuthSession, Credentials};

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/logout", get(self::get::logout))
        .route("/signup", post(self::post::signup))
}

mod post {
    use super::*;

    pub async fn signup(
        mut auth_session: AuthSession,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        tracing::info!("signup");
        auth_session
            .backend
            .add_user(
                creds.clone().password,
                creds.clone().username,
                creds.clone().signup.unwrap().grade,
                creds.clone().signup.unwrap().realname,
            )
            .await;
        tracing::info!("after add_user");
        Redirect::to("/login").into_response()
    }

    pub async fn login(
        mut auth_session: AuthSession,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if let Some(ref next) = creds.next {
            Redirect::to(next).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    }
}

mod get {
    use super::*;

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
