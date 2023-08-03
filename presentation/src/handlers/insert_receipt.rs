use actix_web::http::header;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use dependency_injection::FeatureContainer;
use crate::input::ReceiptDto;
use std::fmt::Error;

use crate::error::ApiError;

#[post("/receipt")]
pub async fn insert_receipt(
    data: Data<FeatureContainer>,
    req: HttpRequest,
    receipt: Json<ReceiptDto>,
) -> Result<HttpResponse, ApiError> {
    let from = req.headers().get("from").ok_or_else(|| {
        ApiError::new(
            "Please pass a correct header".to_string(),
            "There is no header from".to_string(),
            actix_web::http::StatusCode::BAD_REQUEST,
        )
    })?.to_str().map_err(|err| {
        ApiError::new(
            "Please pass a correct header".to_string(),
            err.to_string(),
            actix_web::http::StatusCode::BAD_REQUEST,
        )
    })?.to_string();

    match data.insert_receipt(from).execute(receipt.into_inner().into()).await {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(ApiError::new(
            "Error inserting receipt".to_string(),
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

