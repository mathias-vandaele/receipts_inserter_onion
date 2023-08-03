mod provider;
mod mongo_impl;

pub use provider::Provider;
use mongo_impl::mongo_impl_receipt::MongoCollection;