use std::error::Error;
use domain::Receipt;
use async_trait::async_trait;
use domain::repository::receipt_repository::ReceiptRepository;
use mongodb::{bson::doc, Collection, bson::oid, bson};

pub struct MongoCollection {
    collection: Collection<Receipt>,
}

impl MongoCollection {
    pub fn new(collection: Collection<Receipt>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl ReceiptRepository for MongoCollection {
    async fn find_by_id(&self, id: String) -> Result<Receipt, Box<dyn Error>> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await?.ok_or_else( || "No receipt found".into())
    }

    async fn insert(&self, receipt: Receipt) -> Result<(), Box<dyn Error>> {
        self.collection.insert_one(receipt, None).await?;
        Ok(())
    }
}