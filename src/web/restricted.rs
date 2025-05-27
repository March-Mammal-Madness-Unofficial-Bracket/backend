use axum::{http::StatusCode, response::IntoResponse, routing::post, routing::get, Router, extract, Json};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/restricted", get(self::get::restricted))
        .route("/update_bracket", post(self::post::update_bracket))
        .route("/new_bracket", post(self::post::new_bracket))
}

mod get {
    use super::*;

    pub async fn restricted(auth_session: AuthSession) -> impl IntoResponse {
        StatusCode::OK.into_response()
    }
}

mod post {
    use super::*;

    pub async fn update_bracket(auth_session: AuthSession, extract::Json(payload): extract::Json<crate::bracket::Bracket>) -> impl IntoResponse {
        
    }

    pub async fn new_bracket(auth_session: AuthSession, extract::Json(payload): extract::Json<Vec<String>>) -> impl IntoResponse {

    }
}
