use std::error::Error;
use infra::Provider;
use domain::Receipt;

#[derive(Clone, Debug)]
pub struct FeatureContainer {
    infra_provider: Provider
}

impl FeatureContainer {
    pub async fn new() -> FeatureContainer {
        let infra_provider = Provider::new().await.expect("Could not start");
        FeatureContainer { infra_provider }
    }

    pub async fn insert_receipt(&self, receipt: Receipt) -> Result<(), Box<dyn Error>> {
        
        Ok(())
    }
}