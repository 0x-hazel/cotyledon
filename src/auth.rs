use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Query, http::StatusCode, response::Redirect, routing::{get, post}, Form, Router};
use axum_messages::{Message, Messages};
use fomat_macros::fomat;
use serde::Deserialize;

use crate::users::{AuthSession, LoginCredentials};


#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    messages: Vec<Message>,
        next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod post {
    use askama_axum::Response;

    use crate::users::RegisterCredentials;

    use super::*;

    pub async fn login(mut auth_session: AuthSession, messages: Messages, Form(credentials): Form<LoginCredentials>) -> impl IntoResponse {
        let user = match auth_session.authenticate(credentials.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                messages.error("Invalid credentials");
                let mut login_url = String::from("/login");
                if let Some(next) = credentials.next {
                    login_url = fomat!((login_url)"?next="(next));
                };
                return Redirect::to(&login_url).into_response();
            },
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        messages.success(fomat!("Successfully logged in as "(user.username)));

        if let Some(ref next) = credentials.next {
            Redirect::to(next)
        } else {
            Redirect::to("/")
        }
        .into_response()
    }

    pub async fn register(mut auth_session: AuthSession, messages: Messages, Form(credentials): Form<RegisterCredentials>) -> impl IntoResponse {
        let creds = match auth_session.backend.register(&credentials).await {
            Ok(Some(creds)) => creds,
            Ok(None) => {
                messages.error("Credentials already in use");
                let mut login_url = String::from("/login");
                if let Some(next) = credentials.next {
                    login_url = fomat!((login_url)"?next="(next));
                };
                return Redirect::to(&login_url).into_response();
            },
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        messages.clone().success(fomat!("Registered user "(&credentials.username)));
        return login(auth_session, messages, Form(creds)).await
    }
}

mod get {
    use super::*;

    pub async fn login(messages: Messages, Query(NextUrl{next}): Query<NextUrl>) -> LoginTemplate {
        LoginTemplate {
            messages: messages.into_iter().collect(),
            next,
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}