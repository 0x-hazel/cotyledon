use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Redirect, routing::{get, post}, Form, Router};
use axum_messages::Messages;

use crate::param::PostDetails;
use crate::template::{ProtectedTemplate, PostTemplate};
use crate::users::AuthSession;


pub fn router() -> Router {
    Router::new()
        .route("/", get(self::get::home))
        .route("/post", get(self::get::post))
        .route("/post", post(self::post::post))
}

mod get {
    use super::*;

    pub async fn home(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => ProtectedTemplate {
                messages: messages.into_iter().collect(),
                user,
            }.into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    pub async fn post(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => PostTemplate {
                messages: messages.into_iter().collect(),
                user,
            }.into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

mod post {
    use super::*;

    pub async fn post(auth_session: AuthSession, Form(post): Form<PostDetails>) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => {
                sqlx::query("INSERT INTO posts (user_id, body) VALUES ($1, $2)")
                    .bind(user.id)
                    .bind(post.body)
                    .execute(&auth_session.backend.db)
                    .await
                    .expect("Unable to create new post");
                Redirect::to("/").into_response()
            },
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}