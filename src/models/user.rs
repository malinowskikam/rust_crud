use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}
