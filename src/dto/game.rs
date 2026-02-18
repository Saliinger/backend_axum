use super::user::UserDto;
use chrono::prelude::*;

pub struct GameDto {
    pub id: String,
    pub white_player_id: String,
    pub black_player_id: String,
    pub white_player: UserDto,
    pub black_player: UserDto,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
