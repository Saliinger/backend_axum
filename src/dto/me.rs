use serde::{Deserialize, Serialize};

pub struct MeDto {
	pub username: String,
	pub bio: String,
	pub avatar: String,

}

#[derive(Clone, Deserialize, Serialize)]
pub struct PatchMeDto {
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}
