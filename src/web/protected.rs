use axum::{
    extract, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router,
};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new()
        .route("/leaderboard", get(self::get::leaderboard))
        .route("/bracket", get(self::get::bracket))
        .route("/bracket", post(self::post::bracket))
}

mod get {

    use super::*;

    pub async fn bracket(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => {
                Json(auth_session.backend.get_bracket(user).await.unwrap()).into_response()
            }
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn leaderboard(auth_session: AuthSession) -> impl IntoResponse {
        let mut leaderboard = auth_session.backend.get_leaderboard().await.unwrap();
        for users in &mut leaderboard {
            users.realname = None;
        }
        match auth_session.user {
            Some(_user) => Json(leaderboard).into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

mod post {
    use super::*;
    pub async fn bracket(
        auth_session: AuthSession,
        extract::Json(payload): extract::Json<crate::bracket::Bracket>,
    ) {
        tracing::info!("changing user bracket");
        if let Some(user) = auth_session.user {
            auth_session
                .backend
                .add_bracket(payload, user)
                .await
                .unwrap();
            tracing::info!("user bracket changed");
        }
    }
}
