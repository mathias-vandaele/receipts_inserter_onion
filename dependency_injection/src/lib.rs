use std::error::Error;
use infra::Provider;
use domain::Receipt;
use use_case::tasks::insert_receipt::InsertReceipt;
use use_case::tasks::get_receipt::GetReceipt;

#[derive(Clone, Debug)]
pub struct FeatureContainer {
    infra_provider: Provider
}

impl FeatureContainer {
    pub async fn new() -> FeatureContainer {
        let infra_provider = Provider::new().await.expect("Could not start");
        FeatureContainer { infra_provider }
    }

    pub fn insert_receipt(&self, from : String) -> InsertReceipt {
        InsertReceipt::new(Box::new(self.infra_provider.provide_mongo_collection(from)))
    }

    pub fn get_receipt(&self, from : String) -> GetReceipt {
        GetReceipt::new(Box::new(self.infra_provider.provide_mongo_collection(from)))
    }
}