use crate::{
    config::Config,
    domain::{game::service::GameService, me::service::MeService, user::service::UserService},
    infra::redis::Redis,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub redis: Redis,
    pub user_service: Arc<UserService>,
    pub game_service: Arc<GameService>,
    pub me_service: Arc<MeService>,
}
