use sea_orm::DatabaseConnection;

pub struct FriendsService {
    db: DatabaseConnection,
}

impl FriendsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
