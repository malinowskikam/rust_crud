use actix_web::Result;
use sqlx::PgPool;

use crate::{
    core::{auth::verify_password, errors::ApiError},
    dto::login::LoginPayload,
    models::user::User,
};

pub async fn login(pool: &PgPool, payload: &LoginPayload) -> Result<User> {
    let result = sqlx::query_as!(
        User,
        r#"select id, username, password_hash from users where username = $1;"#,
        payload.username
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(user)) => {
            verify_password(&payload.password, &user.password_hash)?;
            Ok(user)
        }
        Ok(None) => Err(ApiError::Unauthorized.into()),
        Err(_) => Err(ApiError::InternalServerError.into()),
    }
}
