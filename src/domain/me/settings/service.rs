use sea_orm::DatabaseConnection;

pub struct SettingsService {
    db: DatabaseConnection,
}

impl SettingsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
