use askama::Template;
use axum_messages::Message;

use crate::model::{DisplayUser, Thread};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

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
#[template(path = "dash.html")]
pub struct DashTemplate {
    pub messages: Vec<Message>,
    pub user: DisplayUser,
    pub posts: Vec<Thread>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub messages: Vec<Message>,
    pub user: DisplayUser,
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    pub logged_in: bool,
    pub following: bool,
    pub user: DisplayUser,
    pub posts: Vec<Thread>,
}