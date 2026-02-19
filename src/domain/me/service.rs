use super::repository::MeRepository;
use crate::app_error::AppError;
use crate::dto::user::UserDto;
use std::sync::Arc;

use super::{
    friends::service::FriendsService, settings::service::SettingsService,
    stats::service::StatsService,
};

use crate::dto::me::PatchMeDto;

#[derive(Clone)]
pub struct MeService {
    repo: Arc<dyn MeRepository>,
    pub friends: Arc<FriendsService>,
    pub stats: Arc<StatsService>,
    pub settings: Arc<SettingsService>,
}

impl MeService {
    pub fn new(
        repo: Arc<dyn MeRepository>,
        friends: Arc<FriendsService>,
        stats: Arc<StatsService>,
        settings: Arc<SettingsService>,
    ) -> Self {
        Self {
            repo,
            friends,
            stats,
            settings,
        }
    }

    pub async fn get_me(&self, user_id: String) -> Result<UserDto, AppError> {
        let user = self.repo.get_by_id(user_id).await?;
        Ok(UserDto::from(user))
    }

    pub async fn patch_me(&self, user_id: String, user: PatchMeDto) -> Result<UserDto, AppError> {
        // Repository returns a UserDto after applying the patch
        let user_dto = self.repo.patch_by_id(user_id, user).await?;
        Ok(user_dto)
    }

    pub async fn delete_me(&self, user_id: String) -> Result<UserDto, AppError> {
        // Get full user DTO (with stats) before deleting, so we can return it
        let tuple = self.repo.get_by_id(user_id.clone()).await?;
        let dto = UserDto::from(tuple);
        // perform deletion (returns deleted id)
        let _ = self.repo.delete_by_id(user_id).await?;
        Ok(dto)
    }
}
