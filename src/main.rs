use anyhow::Result;
use app::App;

mod config;
mod app;
mod users;
mod protected;
mod auth;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load()?;
    App::new(config).await?.serve().await
}
