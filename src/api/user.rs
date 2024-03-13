use crate::{
    core::state::AppState,
    dto::user::UserPayload,
    models::user::User,
    services::user::{create_user, get_all_users, get_user, update_user},
};
use actix_web::{get, post, put, web, HttpResponse, Responder, Result};
use uuid::Uuid;

#[get("/")]
async fn index_handler(data: web::Data<AppState>, _: User) -> Result<impl Responder> {
    let rows = get_all_users(&data.db_pool).await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[get("/{uuid}")]
async fn get_handler(
    data: web::Data<AppState>,
    _: User,
    uuid: web::Path<Uuid>,
) -> Result<impl Responder> {
    let row = get_user(&data.db_pool, &uuid).await?;
    Ok(HttpResponse::Ok().json(row))
}

#[post("/")]
async fn post_handler(
    data: web::Data<AppState>,
    _: User,
    user: web::Json<UserPayload>,
) -> Result<impl Responder> {
    create_user(&data.db_pool, &user).await?;
    Ok(HttpResponse::Created().finish())
}

#[put("/{uuid}")]
async fn put_handler(
    data: web::Data<AppState>,
    _: User,
    uuid: web::Path<Uuid>,
    user: web::Json<UserPayload>,
) -> Result<impl Responder> {
    update_user(&data.db_pool, &uuid, &user).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn users_service() -> actix_web::Scope {
    web::scope("/user")
        .service(index_handler)
        .service(get_handler)
        .service(post_handler)
        .service(put_handler)
}
