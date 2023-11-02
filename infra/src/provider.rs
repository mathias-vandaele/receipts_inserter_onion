use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;
use std::ops::DerefMut;
use mongodb::{Client, Database, Collection};
use mongodb::options::{ClientOptions, ResolverConfig};
use domain::Receipt;
use domain::repository::receipt_repository::ReceiptRepository;
use crate::mongo_impl::mongo_impl_receipt::MongoCollection;

#[derive(Clone, Debug)]
pub struct Provider{
    mongo_client: Database,
}

impl Provider{
    pub async fn new() -> Result<Provider, Box<dyn std::error::Error>> {
        let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

        let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;
        let database = client.default_database().expect("could not connect to database");
        Ok(Provider { mongo_client: database })
    }

    pub fn provide_mongo_collection(&self, collection : String) -> impl ReceiptRepository + Send + Sync {
        MongoCollection::new(self.mongo_client.collection::<Receipt>(&collection))
    }
    
}
