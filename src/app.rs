use anyhow::Result;
use axum_login::{login_required, tower_sessions::ExpiredDeletion, AuthManagerLayerBuilder};
use axum_messages::MessagesManagerLayer;
use sqlx::{any::install_default_drivers, AnyPool, SqlitePool};
use tokio::{signal, task::AbortHandle};
use tower_sessions::{cookie::{time::Duration, Key}, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

use crate::{auth, config::Config, protected, users::Backend};

pub struct App {
    db: AnyPool,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        install_default_drivers();
        let db = AnyPool::connect_lazy(&config.database_url)?;
        sqlx::migrate!().run(&db).await?;
        Ok(Self { db })
    }
    pub async fn serve(self) -> Result<()> {
        // Sqlite only session storage
        let session_store = SqliteStore::new(SqlitePool::connect(":memory:").await?);
        session_store.migrate().await?;

        let deletion = tokio::task::spawn(
            session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60))
        );

        let key = Key::generate();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        let backend = Backend::new(self.db);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app = protected::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .merge(auth::router())
            .layer(MessagesManagerLayer)
            .layer(auth_layer);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(deletion.abort_handle()))
            .await?;
        deletion.await??;
        Ok(())
    }
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}