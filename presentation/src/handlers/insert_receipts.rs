use actix_web::http::header;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use dependency_injection::FeatureContainer;
use crate::input::ReceiptDto;
use std::fmt::Error;

use crate::error::ApiError;

#[post("/insert")]
pub async fn insert_receipt(
    data: Data<FeatureContainer>,
    req: HttpRequest,
    receipt: web::Json<ReceiptDto>,
) -> Result<HttpResponse, ApiError> {
    let from = req.headers().get("from").ok_or_else(|| {
        ApiError::new(
            "Please pass a correct header".to_string(),
            "There is no header from".to_string(),
            2547,
        )
    })?;

    data.insert_receipt(receipt.into_inner().into()).await;

    Ok(HttpResponse::Accepted().finish())
}
