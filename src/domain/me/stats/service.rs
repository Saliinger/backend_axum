use sea_orm::DatabaseConnection;

pub struct StatsService {
    db: DatabaseConnection,
}

impl StatsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
