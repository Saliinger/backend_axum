use crate::app_error::AppError;
use crate::dto::{me::PatchMeDto, user::UserDto};
use crate::entities::{user, user_stats};

#[async_trait::async_trait]
pub trait MeRepository: Send + Sync {
    async fn get_by_id(
        &self,
        id: String,
    ) -> Result<(user::Model, Option<user_stats::Model>), AppError>;
    async fn patch_by_id(&self, id: String, update: PatchMeDto) -> Result<UserDto, AppError>;
    async fn delete_by_id(&self, id: String) -> Result<String, AppError>;
}