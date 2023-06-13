mod handlers;
mod error;
pub mod input;

use dependency_injection::FeatureContainer;
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let features = FeatureContainer::new().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(features.clone()))
            .service(handlers::insert_receipts::insert_receipt)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
