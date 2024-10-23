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

#[derive(Clone, Debug, Deserialize)]
pub struct PostDetails {
    pub summary: String,
    pub body: String,
    pub responding: Option<i64>,
}

#[derive(Clone, Deserialize)]
pub struct FollowDetails {
    pub name: String,
    pub id: i64,
}

#[derive(Deserialize)]
pub struct PostComposeDetails {
    pub responding: Option<i64>,
}