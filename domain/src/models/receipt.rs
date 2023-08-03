use serde::{Serialize, Deserialize};
use crate::models::item::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub id : String,
    pub items : Vec<Item>
}
