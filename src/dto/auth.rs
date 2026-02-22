use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterDto {
    name: String,
    username: String,
	email: String,
    bio: String,
    password_hash: String,
}
