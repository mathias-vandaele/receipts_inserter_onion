use domain::repository::receipt_repository::ReceiptRepository;

pub struct GetReceipt {
    collection: Box<dyn ReceiptRepository>,
}

impl GetReceipt {
    pub fn new(collection: Box<dyn ReceiptRepository>) -> GetReceipt {
        GetReceipt {
            collection
        }
    }

    pub async fn execute(&self, id: String) -> Result<domain::Receipt, Box<dyn std::error::Error>> {
        self.collection.find_by_id(id).await
    }
}