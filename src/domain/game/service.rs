use crate::infra::repositories::game_repository::GameRepository;

#[derive(Clone)]
pub struct GameService {
    game_repository: GameRepository,
}

impl GameService {
    pub fn new(game_repository: GameRepository) -> Self {
        Self { game_repository }
    }
}
