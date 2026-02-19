use crate::app_error::AppError;
use crate::entities::{user, user_stats};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<(user::Model, Option<user_stats::Model>)>, AppError>;
    async fn get_by_id(
        &self,
        id: String,
    ) -> Result<(user::Model, Option<user_stats::Model>), AppError>;
}
