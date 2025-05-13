use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/restricted", get(self::get::restricted))
}

mod get {
    use super::*;

    pub async fn restricted(auth_session: AuthSession) -> impl IntoResponse {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
