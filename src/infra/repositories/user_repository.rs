use crate::app_error::AppError;
use crate::entities::{user, user_stats};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<(user::Model, Option<user_stats::Model>)>, AppError> {
        let users = user::Entity::find()
            .find_also_related(user_stats::Entity)
            .all(&self.db)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(users)
    }

    pub async fn get_by_id(
        &self,
        id: i32,
    ) -> Result<(user::Model, Option<user_stats::Model>), AppError> {
        let user = user::Entity::find_by_id(id.to_string())
            .find_also_related(user_stats::Entity)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        match user {
            Some(u) => Ok(u),
            None => Err(AppError::NotFound),
        }
    }
}
