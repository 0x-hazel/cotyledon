use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::{generate_hash, verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, AnyPool};
use thiserror::Error;
use tokio::task;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    id: i64,
    pub username: String,
    password: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone, Deserialize)]
pub struct LoginCredentials {
    pub username: String, 
    pub password: String,
    pub next: Option<String>,
}

impl LoginCredentials {
    pub fn from(creds: &RegisterCredentials) -> Self {
        LoginCredentials {
            username: creds.username.clone(),
            password: creds.password.clone(),
            next: creds.next.clone(),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct RegisterCredentials {
    pub email: String,
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Backend {
    db: AnyPool
}

impl Backend {
    pub fn new(db: AnyPool) -> Self {
        Self { db }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError)
}

impl Backend {
    pub async fn register(&self, credentials: &RegisterCredentials) -> Result<Option<LoginCredentials>, Error> {
        let existing = sqlx::query("SELECT * FROM users WHERE username = $1 OR email = $2")
            .bind(&credentials.username)
            .bind(&credentials.email)
            .fetch_optional(&self.db)
            .await?;
        match existing {
            Some(_) => {
                Ok(None)
            },
            None => {
                let password_hash = generate_hash(&credentials.password);
                sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3);")
                    .bind(&credentials.username)
                    .bind(&credentials.email)
                    .bind(password_hash)
                    .execute(&self.db)
                    .await?;
                Ok(Some(LoginCredentials::from(credentials)))
            }
        }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = LoginCredentials;
    type Error = Error;

    async fn authenticate(&self, credentials: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("SELECT * FROM users WHERE username = ?")
            .bind(credentials.username)
            .fetch_optional(&self.db)
            .await?;
        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(credentials.password, &user.password).is_ok()))
        }).await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;
        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;