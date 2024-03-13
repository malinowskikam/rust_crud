use actix_web::ResponseError;
use std::fmt;

#[derive(Debug)]
pub struct UuidParseFailed;

impl ResponseError for UuidParseFailed {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().finish()
    }
}

impl fmt::Display for UuidParseFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse UUID")
    }
}

pub fn parse_uuid(s: &str) -> Result<uuid::Uuid, UuidParseFailed> {
    uuid::Uuid::parse_str(s).map_err(|_| UuidParseFailed)
}
