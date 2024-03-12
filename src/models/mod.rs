use core::fmt;

use actix_web::{ResponseError, http::StatusCode, HttpResponse};

pub mod user;

#[derive(Debug)]
enum QueryError {
    NotFound,
    Internal,
}

impl ResponseError for QueryError {
    fn status_code(&self) -> StatusCode {
        match self {
            QueryError::NotFound => StatusCode::NOT_FOUND,
            QueryError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            QueryError::NotFound => HttpResponse::NotFound().finish(),
            QueryError::Internal => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::NotFound => write!(f, "Not found"),
            QueryError::Internal => write!(f, "Internal error"),
        }
    }
}