use crate::db;
use sea_orm::DatabaseConnection;

use super::config;

#[derive(Clone)]
pub struct AppState {
    config: config::Config,
    db: DatabaseConnection,
}

impl AppState {
    pub async fn new(config: config::Config) -> AppState {
        let db = db::db_init(config.database_url).await?;
        Self { config, db }
    }

    pub fn get_port(&self) -> &u16 {
        &self.config.port
    }
}
