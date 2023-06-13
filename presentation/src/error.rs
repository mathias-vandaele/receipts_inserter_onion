use std::fmt::{Debug, Display, Formatter};
use actix_web::ResponseError;

pub struct ApiError{
    message: String,
    cause: String,
    code: u16
}

impl ApiError {
    pub fn new(message: String, cause: String, code: u16) -> ApiError {
        ApiError { message, cause, code }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} - {} - {}", self.code, self.message, self.cause)
    }
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} - {} - {}", self.code, self.message, self.cause)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}