use sea_orm::DatabaseConnection;

use super::config;

#[derive(Clone)]
pub struct AppState {
    config: config::Config,
    db: DatabaseConnection,
    redis: redis::Client,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: config::Config, redis: redis::Client) -> AppState {
        Self { config, db, redis }
    }

    pub fn get_port(&self) -> u16 {
        let port: u16 = self.config.port.clone();
        port
    }

    pub fn get_db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn get_redis(&self) -> &redis::Client {
        &self.redis
    }
}
