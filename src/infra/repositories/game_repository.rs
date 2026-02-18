use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct GameRepository {
    db: DatabaseConnection,
}

impl GameRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
