use super::config;
use crate::domain::game::service::GameService;
use crate::domain::user::service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub redis: redis::Client,
    pub user_service: UserService,
    pub game_service: GameService,
}
