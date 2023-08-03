use domain::repository::receipt_repository::ReceiptRepository;

pub struct InsertReceipt{
    collection :  Box<dyn ReceiptRepository>
}

impl InsertReceipt{
    pub fn new(collection : Box<dyn ReceiptRepository>) -> InsertReceipt{
        InsertReceipt{
            collection
        }
    }

    pub async fn execute(&self, receipt : domain::Receipt) -> Result<(), Box<dyn std::error::Error>>{
        self.collection.insert(receipt).await
    }
}