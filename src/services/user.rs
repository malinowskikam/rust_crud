use actix_web::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    core::errors::ApiError,
    dto::user::UserPayload,
    models::user::User,
};

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>> {
    let result = sqlx::query_as!(User, r#"select id, username, password_hash from users;"#)
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) => Ok(rows),
        Err(_) => Err(ApiError::InternalServerError.into()),
    }
}

pub async fn get_user(pool: &PgPool, id: &Uuid) -> Result<User> {
    let result = sqlx::query_as!(
        User,
        r#"select id, username, password_hash from users where id = $1;"#,
        id
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(row)) => Ok(row),
        Ok(None) => Err(ApiError::NotFound.into()),
        Err(_) => Err(ApiError::InternalServerError.into()),
    }
}

pub async fn create_user(pool: &PgPool, user: &UserPayload) -> Result<()> {
    let password_hash = generate_password_hash(&user.password)?;
    let result = sqlx::query!(
        r#"INSERT INTO users (username, password_hash) VALUES ($1, $2);"#,
        user.username,
        password_hash
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError::InternalServerError.into()),
    }
}

pub async fn update_user(pool: &PgPool, id: &Uuid, user: &UserPayload) -> Result<()> {
    let password_hash = generate_password_hash(&user.password)?;
    let result = sqlx::query!(
        r#"UPDATE users SET username = $1, password_hash = $2 WHERE id = $3;"#,
        user.username,
        password_hash,
        id
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError::InternalServerError.into()),
    }
}

fn generate_password_hash(password: &str) -> Result<String> {
    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &password_salt)
        .map_err(|_| ApiError::InternalServerError)?
        .to_string();

    Ok(password_hash)
}