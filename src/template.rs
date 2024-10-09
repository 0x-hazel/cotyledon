use askama::Template;
use axum_messages::Message;

use crate::model::{Post, User};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub messages: Vec<Message>,
    pub next: Option<String>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub messages: Vec<Message>,
    pub next: Option<String>,
}

#[derive(Template)]
#[template(path = "protected.html")]
pub struct ProtectedTemplate {
    pub messages: Vec<Message>,
    pub user: User,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub messages: Vec<Message>,
    pub user: User,
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub user: User,
    pub posts: Vec<Post>,
}