use std::{borrow::Cow, fmt::Debug};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{AnyPool, FromRow};

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String
}

impl AuthUser {
    pub async fn get_display(&self, db: &AnyPool) -> Result<DisplayUser> {
        Ok(
            sqlx::query_as("SELECT id, username, display_name, bio FROM users WHERE id = $1")
                .bind(self.id)
                .fetch_one(db)
                .await?
        )
    }
}

impl Debug for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthUser")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password", &"[password]")
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct DisplayUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub bio: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct RawPost {
    pub id: i64,
    pub thread: Option<String>,
    pub created: String,
    pub summary: Option<String>,
    pub body: String,
}

impl RawPost {
    pub async fn into(self, db: &AnyPool) -> Result<Thread> {
        let contents = match self.thread {
            Some(t) => {
                // Rather hacky here, but should work fine as long as thread data is properly sanitised
                // Hopefully will be fixable if/when this project migrates to a more cohesive database solution
                // (probably seaORM or something similar)
                let args = t.split('/').map(Cow::from).reduce(|mut acc, s| {
                    acc.to_mut().push_str(", ");
                    acc.to_mut().push_str(&s);
                    acc
                }).unwrap_or_default();
                let query = format!("SELECT created, summary, body FROM posts WHERE id IN ({})", args);
                println!("Querying: {}", query);
                let mut result = sqlx::query_as(&query)
                    .fetch_all(db)
                    .await?;
                result.push(Post {
                    created: self.created.clone(),
                    summary: self.summary,
                    body: self.body
                });
                result
            },
            None => vec![Post {
                created: self.created.clone(),
                summary: self.summary,
                body: self.body
            }]
        };
        let tags: Vec<(String, )> = sqlx::query_as("SELECT tag FROM tags INNER JOIN postTags ON postTags.tag_id = tags.id WHERE postTags.post_id = $1")
            .bind(self.id)
            .fetch_all(db)
            .await?;
        Ok(
            Thread {
                timestamp: self.created,
                contents,
                tags: tags.into_iter().map(|x| x.0).collect()
            }
        )
    }
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Post {
    pub created: String,
    pub summary: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Thread {
    pub timestamp: String,
    pub contents: Vec<Post>,
    pub tags: Vec<String>,
}