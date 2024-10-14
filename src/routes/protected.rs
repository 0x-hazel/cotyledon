use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Redirect, routing::{get, post}, Form, Router};
use axum_messages::Messages;

use crate::param::{FollowDetails, PostDetails};
use crate::template::{DashTemplate, PostTemplate};
use crate::authentication::AuthSession;


pub fn router() -> Router {
    Router::new()
        .route("/dash", get(self::get::home))
        .route("/post", get(self::get::post))
        .route("/post", post(self::post::post))
        .route("/follow", post(self::post::follow))
}

mod get {
    use super::*;

    pub async fn home(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => DashTemplate {
                messages: messages.into_iter().collect(),
                user: match user.get_display(&auth_session.backend.db).await {
                    Ok(u) => u,
                    Err(e) => {
                        println!("{:?}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                },
                posts: match auth_session.backend.get_dash_contents(user.id).await {
                    Ok(c) => c,
                    Err(e) => {
                        println!("{:?}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
            }.into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    pub async fn post(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => PostTemplate {
                messages: messages.into_iter().collect(),
                user: match user.get_display(&auth_session.backend.db).await {
                    Ok(u) => u,
                    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                },
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
                Redirect::to("/dash").into_response()
            },
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    pub async fn follow(auth_session: AuthSession, Form(follow): Form<FollowDetails>) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => {
                match sqlx::query("INSERT INTO follows (follower, followee, is_accepted) VALUES ($1, $2, 1)")
                    .bind(user.id)
                    .bind(follow.id)
                    .execute(&auth_session.backend.db)
                    .await {
                        Ok(_) => Redirect::to(&format!("/user/{}", follow.name)).into_response(),
                        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            },
            None => StatusCode::UNAUTHORIZED.into_response()
        }
    }
}