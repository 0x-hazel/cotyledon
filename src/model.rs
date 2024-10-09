use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Post {
    pub created: String,
    pub body: String,
}