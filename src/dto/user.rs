use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
}
