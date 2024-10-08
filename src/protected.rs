use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Redirect, routing::{get, post}, Form, Router};
use axum_messages::{Message, Messages};
use serde::Deserialize;

use crate::users::{AuthSession, User};

#[derive(Template)]
#[template(path = "protected.html")]
struct ProtectedTemplate<'a> {
    messages: Vec<Message>,
    username: &'a str,
    bio: &'a str,
}

#[derive(Clone, Deserialize)]
struct PostDetails {
    body: String,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    messages: Vec<Message>,
    user: User,
}

pub fn router() -> Router<()> {
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
                username: &user.username,
                bio: &user.bio
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