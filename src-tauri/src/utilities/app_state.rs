use sea_orm::DbConn;
use std::sync::Arc;
use tokio::sync::RwLock;

/// AppState is shared across Tauri commands
#[derive(Clone)]
pub struct AppState {
    pub app_db: Arc<RwLock<Option<DbConn>>>,
    pub connection: Arc<RwLock<Option<Connection>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_db: Arc::new(RwLock::new(None)),
            connection: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_connection(&self, new_connection: Connection) {
        let mut existing_connection = self.connection.write().await;
        *existing_connection = Some(new_connection);
    }

    pub async fn get_connection(&self) -> Option<Connection> {
        self.connection.read().await.clone()
    }

    pub async fn get_app_db_connection(&self) -> Result<DbConn, String> {
        let read_guard = self.app_db.read().await;
        read_guard
            .clone()
            .ok_or_else(|| "App DB not initialized yet".to_string())
    }

    pub async fn set_app_db_connection(&self, db: DbConn) -> () {
        let mut write_guard = self.app_db.write().await;
        *write_guard = Some(db);
    }
}

#[derive(Clone)]
pub struct Connection {
    pub connection_id: i64,
    pub name: String,
}

impl Connection {
    pub fn new(connection_id: i64, name: String) -> Self {
        Self {
            connection_id,
            name,
        }
    }
}
