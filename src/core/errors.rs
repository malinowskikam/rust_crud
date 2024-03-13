use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest,
    InternalServerError,
    Unauthorized,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            ApiError::BadRequest => HttpResponse::BadRequest().finish(),
            ApiError::InternalServerError => HttpResponse::InternalServerError().finish(),
            ApiError::Unauthorized => HttpResponse::Unauthorized().append_header(("WWW-Authenticate", r#"Basic realm="User Visible Realm", charset="UTF-8""#)).finish(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Not Found"),
            ApiError::BadRequest => write!(f, "Bad Request"),
            ApiError::InternalServerError => write!(f, "Internal Server Error"),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

pub fn path_error_handler(
    _err: actix_web::error::PathError,
    _req: &actix_web::HttpRequest,
) -> actix_web::error::Error {
    ApiError::BadRequest.into()
}

pub fn json_error_handler(
    _err: actix_web::error::JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::error::Error {
    ApiError::BadRequest.into()
}
