use axum::{Json, http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};
use serde::{Serialize, Deserialize};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/leaderboard", get(self::get::leaderboard)).route("/bracket", get(self::get::bracket)).route("/bracket", post(self::post::bracket))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BracketResponse {
    score: i64,
    bracket: String,
}

mod get {
    use axum::response::Html;

    use super::*;

    pub async fn bracket(auth_session: AuthSession) -> Json<BracketResponse>{
        Json(BracketResponse { score: 1, bracket: "example".to_string()})
    }

    pub async fn leaderboard(auth_session: AuthSession) -> Json<Vec<BracketResponse>>{
        Json(vec![BracketResponse { score: 1, bracket: "example".to_string()}])

    }
}

mod post {
    use super::*;
    pub async fn bracket(auth_session: AuthSession) {

    }
}

