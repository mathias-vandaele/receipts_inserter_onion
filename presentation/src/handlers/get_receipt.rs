use actix_web::{HttpRequest, HttpResponse, web, get};
use actix_web::web::Data;
use dependency_injection::FeatureContainer;
use domain::Receipt;
use crate::error::ApiError;
use crate::input::ReceiptDto;

#[get("/receipt/{id}")]
pub async fn get_receipt(
    data: Data<FeatureContainer>,
    req: HttpRequest,
    id: web::Path<String>,
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

    match data.get_receipt(from).execute(id.into_inner()).await {
        Ok(receipt) => Ok(HttpResponse::Ok().json(ReceiptDto::from(receipt))),
        Err(err) => Err(ApiError::new(
            "Error inserting receipt".to_string(),
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
