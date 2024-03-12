use actix_web::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{time::OffsetDateTime, Uuid},
    PgPool,
};

use super::QueryError;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>> {
    if let Ok(rows) = sqlx::query_as!(User, r#"SELECT id, username, created_at FROM users;"#)
        .fetch_all(pool)
        .await
    {
        Ok(rows)
    } else {
        Err(QueryError::Internal.into())
    }
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User> {
    if let Ok(row) = sqlx::query_as!(
        User,
        r#"SELECT id, username, created_at FROM users WHERE id = $1;"#,
        id
    )
    .fetch_optional(pool)
    .await
    {
        if let Some(row) = row {
            return Ok(row);
        } else {
            return Err(QueryError::NotFound.into());
        }
    } else {
        return Err(QueryError::Internal.into());
    }
}

pub async fn create_user(pool: &PgPool, user: &User) -> Result<()> {
    if let Ok(_) = sqlx::query!(
        r#"INSERT INTO users (username) VALUES ($1);"#,
        user.username
    )
    .execute(pool)
    .await
    {
        Ok(())
    } else {
        Err(QueryError::Internal.into())
    }
}
