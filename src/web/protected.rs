use axum::{Json, http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::protected))
}

pub struct BracketResponse {
    score: i64,
    bracket: String,
}

mod get {
    use axum::response::Html;

    use super::*;

    pub async fn bracket(auth_session: AuthSession) -> Json<BracketResponse> {
        Json(BracketResponse { score: 1, bracket: "example".to_string()})
    }

    pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
           StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
