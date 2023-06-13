use std::collections::HashMap;
use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ResolverConfig};

#[derive(Clone, Debug)]
pub struct Provider{
    mongo_client: Client,
    collections: HashMap<String, Database>,
}

impl Provider{
    pub async fn new() -> Result<Provider, Box<dyn std::error::Error>> {
        //let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let client_uri = "mongodb://admin:your_password@localhost:27017/receipts";

        let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
        let client = Client::with_options(options)?;
        Ok(Provider { mongo_client: client, collections: HashMap::<String, Database>::new() })
    }
}
