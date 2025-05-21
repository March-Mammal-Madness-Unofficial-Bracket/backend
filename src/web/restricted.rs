use axum::{http::StatusCode, response::IntoResponse, routing::post, routing::get, Router};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/restricted", get(self::get::restricted))
        .route("/update_bracket", post(self::post::update_bracket))
        .route("/new_bracket", post(self::post::new_bracket))
}

mod get {
    use super::*;

    pub async fn restricted(auth_session: AuthSession) -> impl IntoResponse {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

mod post {
    use super::*;

    pub async fn update_bracket() -> impl IntoResponse {

    }

    pub async fn new_bracket() -> impl IntoResponse {

    }
}
