use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Query, http::StatusCode, response::Redirect, routing::{get, post}, Form, Router};
use axum_messages::{Message, Messages};
use fomat_macros::{fomat, pintln};
use serde::Deserialize;

use crate::users::{AuthSession, LoginCredentials};


#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    messages: Vec<Message>,
        next: Option<String>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/register", post(self::post::register))
        .route("/register", get(self::get::register))
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

async fn _login(mut auth_session: AuthSession, messages: Messages, creds: LoginCredentials) -> Result<Redirect, StatusCode> {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            messages.error("Invalid credentials");
            let mut login_url = String::from("/login");
            if let Some(next) = creds.next {
                login_url = fomat!((login_url)"?next="(next));
            };
            return Ok(Redirect::to(&login_url))
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
    if auth_session.login(&user).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    pintln!("Login credentials processed successfully");
    messages.success(fomat!("Successfully logged in as "(user.username)));
    if let Some(ref next) = creds.next {
        return Ok(Redirect::to(next));
    } else {
        return Ok(Redirect::to("/"));
    }
}

mod post {
    use fomat_macros::pintln;

    use crate::users::RegisterCredentials;

    use super::*;

    pub async fn login(auth_session: AuthSession, messages: Messages, Form(credentials): Form<LoginCredentials>) -> impl IntoResponse {
        match _login(auth_session, messages, credentials).await {
            Ok(r) => r.into_response(),
            Err(c) => c.into_response(),
        }
    }

    pub async fn register(auth_session: AuthSession, messages: Messages, Form(credentials): Form<RegisterCredentials>) -> impl IntoResponse {
        let creds = match auth_session.backend.register(&credentials).await {
            Ok(Some(creds)) => creds,
            Ok(None) => {
                messages.error("Credentials already in use");
                let mut register_url = String::from("/register");
                if let Some(next) = credentials.next {
                    register_url = fomat!((register_url)"?next="(next));
                };
                return Redirect::to(&register_url).into_response();
            },
            Err(e) => {
                pintln!([e]);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        };
        pintln!("Registration credentials acquired successfully");
        messages.clone().success(fomat!("Registered user "(&credentials.username)));
        match _login(auth_session, messages, creds).await {
            Ok(r) => r.into_response(),
            Err(c) => c.into_response(),
        }
    }
}

mod get {
    use super::*;

    pub async fn register(messages: Messages, Query(NextUrl{next}): Query<NextUrl>) -> RegisterTemplate {
        RegisterTemplate {
            messages: messages.into_iter().collect(),
            next,
        }
    }

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