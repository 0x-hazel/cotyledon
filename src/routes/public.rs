use askama_axum::IntoResponse;
use axum::{extract::Path, http::StatusCode, routing::get, Router};
use axum::response::Redirect;
use ::futures::future::join_all;

use crate::{model::Thread, template::HomeTemplate};

use crate::template::UserTemplate;
use crate::authentication::AuthSession;

pub fn router() -> Router {
    Router::new()
        .route("/", get(self::get::home))
        .route("/user/:name", get(self::get::user))
}

mod get {
    use super::*;

    pub async fn home(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(_) => Redirect::to("/dash").into_response(),
            None => HomeTemplate.into_response()
        }
    }

    pub async fn user(auth_session: AuthSession, Path(name): Path<String>) -> impl IntoResponse {
        match auth_session.backend.get_user(&name).await {
            Ok(u) => match u {
                Some(u) => {
                    let posts = auth_session.backend.get_posts(u.id).await;
                    match posts {
                        Ok(posts) => {
                            let posts: Vec<Thread> = join_all(posts.into_iter().map(|x| x.into(&auth_session.backend.db))).await.into_iter().map(|x|x.unwrap()).collect();
                            UserTemplate {
                                user: u,
                                posts,
                            }.into_response()
                        },
                        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                    }
                },
                None => StatusCode::NOT_FOUND.into_response(),
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}