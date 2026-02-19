use super::repository::UserRepository;
use crate::app_error::AppError;
use crate::dto::user::UserDto;

use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<UserDto>, AppError> {
        let users = self.repo.get_all().await?;
        let dtos = users
            .into_iter()
            .map(UserDto::from)
            .collect::<Vec<UserDto>>();
        Ok(dtos)
    }

    pub async fn get_by_id(&self, id: String) -> Result<UserDto, AppError> {
        let user = self.repo.get_by_id(id).await?;
        Ok(UserDto::from(user))
    }
}
