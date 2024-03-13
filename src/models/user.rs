use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::Uuid
};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub password_hash: Option<String>,
}
