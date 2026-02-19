use crate::app_error::AppError;
use crate::entities::game;

#[async_trait::async_trait]
pub trait GameRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<game::Model>, AppError>;
    async fn get_by_id(&self, id: String) -> Result<game::Model, AppError>;
}
