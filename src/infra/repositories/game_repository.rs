use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Clone)]
pub struct GameRepository {
    db: DatabaseConnection,
}

impl GameRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
#[async_trait::async_trait]
impl crate::domain::game::repository::GameRepository for GameRepository {
    async fn get_all(
        &self,
    ) -> Result<Vec<crate::entities::game::Model>, crate::app_error::AppError> {
        let games = crate::entities::game::Entity::find()
            .all(&self.db)
            .await
            .map_err(crate::app_error::AppError::DatabaseError)?;
        Ok(games)
    }

    async fn get_by_id(
        &self,
        id: String,
    ) -> Result<crate::entities::game::Model, crate::app_error::AppError> {
        let game = crate::entities::game::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(crate::app_error::AppError::DatabaseError)?;

        match game {
            Some(g) => Ok(g),
            None => Err(crate::app_error::AppError::NotFound),
        }
    }
}
