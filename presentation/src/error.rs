use std::fmt::{Debug, Display, Formatter};
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::http::header;
use actix_web::web::BytesMut;
use std::fmt::Write;

pub struct ApiError{
    message: String,
    cause: String,
    code: actix_web::http::StatusCode
}

impl ApiError {
    pub fn new(message: String, cause: String, code: actix_web::http::StatusCode) -> ApiError {
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
        self.code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        let mut buf = BytesMut::new();
        let _ = write!(&mut buf, "{{ \"message\" : {}, \"cause\": {} }}", self.message, self.cause);

        res.headers_mut().insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

        res.set_body(BoxBody::new(buf))
    }
}