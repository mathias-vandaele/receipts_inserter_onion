use std::error::Error;

use async_trait::async_trait;
use crate::models::receipt::Receipt;

#[async_trait]
pub trait ReceiptRepository{
    async fn find_by_id(&self, id: String) -> Result<Receipt, Box<dyn Error>>;
    async fn insert(&self, receipt : Receipt) -> Result<(), Box<dyn Error>>;
}