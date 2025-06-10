use axum::{
    extract, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router,
};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new()
        .route("/admin_leaderboard", get(self::get::admin_leaderboard))
        .route("/update_bracket", post(self::post::update_bracket))
        .route("/new_bracket", post(self::post::new_bracket))
}

mod get {
    use super::*;

    pub async fn admin_leaderboard(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => {
                Json(auth_session.backend.get_leaderboard().await.unwrap()).into_response()
            }
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

mod post {
    use super::*;

    pub async fn update_bracket(
        auth_session: AuthSession,
        extract::Json(payload): extract::Json<crate::bracket::Bracket>,
    ) -> impl IntoResponse {
        auth_session.backend.admin_insert_bracket(payload).await;
        auth_session.backend.gen_scores().await;
    }

    pub async fn new_bracket(
        auth_session: AuthSession,
        extract::Json(payload): extract::Json<Vec<String>>,
    ) -> impl IntoResponse {
        let bracket = crate::bracket::Bracket::new(payload);
        auth_session.backend.admin_insert_bracket(bracket).await;
        auth_session.backend.clear_scores().await;
    }
}
