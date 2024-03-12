use crate::{models::user::{get_all_users, get_user}, state::AppState, util::uuid::parse_uuid};
use actix_web::{get, web, HttpResponse, Responder, Result};

#[get("/")]
async fn index(data: web::Data<AppState>) -> Result<impl Responder> {
    let rows = get_all_users(&data.db_pool).await?;
    Ok(HttpResponse::Ok().json(rows))
}

#[get("/{uuid}")]
async fn get(data: web::Data<AppState>, uuid: web::Path<String>) -> Result<impl Responder> {
    let parsed_uuid = parse_uuid(&uuid)?;
    let row = get_user(&data.db_pool, parsed_uuid).await?;
    Ok(HttpResponse::Ok().json(row))
}

pub fn users_service() -> actix_web::Scope {
    web::scope("/users").service(index).service(get)
}
