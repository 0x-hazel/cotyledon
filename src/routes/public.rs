use askama_axum::IntoResponse;
use axum::{extract::Path, http::StatusCode, routing::get, Router};

use crate::template::UserTemplate;
use crate::users::AuthSession;

pub fn router() -> Router {
    Router::new()
        .route("/user/:name", get(self::get::user))
}

mod get {
    use super::*;

    pub async fn user(auth_session: AuthSession, Path(name): Path<String>) -> impl IntoResponse {
        match auth_session.backend.get_user(&name).await {
            Ok(u) => match u {
                Some(u) => {
                    let posts = auth_session.backend.get_posts(u.id).await;
                    match posts {
                        Ok(posts) => UserTemplate {
                            user: u,
                            posts
                        }.into_response(),
                        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                    }
                },
                None => StatusCode::NOT_FOUND.into_response(),
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}