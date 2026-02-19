use crate::app_error::AppError;
use crate::dto::me::PatchMeDto;
use crate::dto::user::UserDto;
use crate::entities::{user, user_stats};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

#[derive(Clone)]
pub struct MeRepository {
    db: DatabaseConnection,
}

impl MeRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl crate::domain::me::repository::MeRepository for MeRepository {
    async fn get_by_id(
        &self,
        id: String,
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

    async fn patch_by_id(&self, id: String, update: PatchMeDto) -> Result<UserDto, AppError> {
        // Fetch the user and optionally related stats
        let pair = user::Entity::find_by_id(id.clone())
            .find_also_related(user_stats::Entity)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        let (model, stats) = match pair {
            Some((m, s)) => (m, s),
            None => return Err(AppError::NotFound),
        };

        let mut active: user::ActiveModel = model.into();

        if let Some(username) = update.username {
            active.name = Set(Some(username));
        }
        if let Some(avatar) = update.avatar {
            active.image = Set(Some(avatar));
        }
        if let Some(bio) = update.bio {
            active.bio = Set(Some(bio));
        }

        let updated = active
            .update(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(UserDto::from((updated, stats)))
    }

    async fn delete_by_id(&self, id: String) -> Result<String, AppError> {
        let existing = user::Entity::find_by_id(id.clone())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        match existing {

            Some(_) => {
                user::Entity::delete_by_id(id.clone())
                    .exec(&self.db)
                    .await
                    .map_err(|e| AppError::DatabaseError(e))?;
                Ok(id)
            }
            None => Err(AppError::NotFound),
        }
    }
}
