use super::repository::GameRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct GameService {
    repo: Arc<dyn GameRepository>,
}

impl GameService {
    pub fn new(repo: Arc<dyn GameRepository>) -> Self {
        Self { repo }
    }
}
