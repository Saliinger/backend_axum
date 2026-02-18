use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub stats: StatsDto,
}

#[derive(Serialize)]
pub struct StatsDto {
    pub id: String,
    pub elo: i32,
    pub global_rank: i32,
    pub wins: i32,
    pub losses: i32,
}

impl IntoResponse for UserDto {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for StatsDto {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl
    From<(
        crate::entities::user::Model,
        Option<crate::entities::user_stats::Model>,
    )> for UserDto
{
    fn from(
        value: (
            crate::entities::user::Model,
            Option<crate::entities::user_stats::Model>,
        ),
    ) -> Self {
        let (user, stats) = value;

        UserDto {
            id: user.id.to_string(),
            name: user.name.unwrap_or_default(),
            email: user.email,
            stats: match stats {
                Some(s) => StatsDto {
                    id: s.id,
                    elo: s.elo,
                    global_rank: s.global_rank,
                    wins: s.wins,
                    losses: s.losses,
                },
                None => StatsDto {
                    id: "none".to_string(),
                    elo: 0,
                    global_rank: 0,
                    wins: 0,
                    losses: 0,
                },
            },
        }
    }
}
