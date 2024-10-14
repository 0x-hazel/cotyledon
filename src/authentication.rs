use anyhow::Result;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::{generate_hash, verify_password};
use sqlx::AnyPool;
use thiserror::Error;
use tokio::task;

use crate::{model::{AuthUser as User, DisplayUser, RawPost, Thread}, param::{LoginCredentials, RegisterCredentials}};

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone, Debug)]
pub struct Backend {
    pub db: AnyPool
}

impl Backend {
    pub fn new(db: AnyPool) -> Self {
        Self { db }
    }

    pub async fn register(&self, credentials: &RegisterCredentials) -> Result<Option<LoginCredentials>, Error> {
        let existing = sqlx::query("SELECT * FROM users WHERE username = $1 OR email = $2")
            .bind(&credentials.username)
            .bind(&credentials.email)
            .fetch_optional(&self.db)
            .await?;
        println!("Existing: {}", existing.is_some());
        match existing {
            Some(_) => {
                println!("Account exists");
                Ok(None)
            },
            None => {
                let password_hash = generate_hash(&credentials.password);
                println!("Account doesn't exist: {}", password_hash);
                sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3);")
                    .bind(&credentials.username)
                    .bind(&credentials.email)
                    .bind(password_hash)
                    .execute(&self.db)
                    .await?;
                println!("Account created");
                Ok(Some(LoginCredentials::from(credentials)))
            }
        }
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<DisplayUser>> {
        let user: Option<DisplayUser> = sqlx::query_as("SELECT id, username, display_name, bio FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.db)
            .await?;
        match user {
            Some(u) => Ok(Some(u)),
            None => Ok(None)
        }
    }

    pub async fn get_posts(&self, user_id: i64) -> Result<Vec<RawPost>> {
        let posts: Vec<RawPost> = sqlx::query_as("SELECT posts.id, users.username, thread, created, summary, body FROM posts INNER JOIN users ON posts.user_id = users.id WHERE user_id = $1 ORDER BY created DESC LIMIT 50")
            .bind(user_id)
            .fetch_all(&self.db)
            .await?;
        Ok(posts)
    }

    pub async fn get_dash_contents(&self, user_id: i64) -> Result<Vec<Thread>> {
        let mut result = Vec::new();
        let follows: Vec<DisplayUser> = sqlx::query_as("SELECT id, username, display_name, bio FROM users INNER JOIN follows ON follows.followee = users.id WHERE follows.follower = $1")
            .bind(user_id)
            .fetch_all(&self.db)
            .await?;
        for follow in follows {
            let posts = self.get_posts(follow.id).await?;
            for post in posts {
                result.push(post.into(&self.db).await?);
            }
        }
        Ok(result)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError)
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = LoginCredentials;
    type Error = Error;

    async fn authenticate(&self, credentials: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(credentials.username)
            .fetch_optional(&self.db)
            .await?;
        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(credentials.password, &user.password).is_ok()))
        }).await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;
        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;