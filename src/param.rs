use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct NextUrl {
    pub next: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct PostDetails {
    pub body: String,
}

#[derive(Clone, Deserialize)]
pub struct FollowDetails {
    pub name: String,
    pub id: i64,
}