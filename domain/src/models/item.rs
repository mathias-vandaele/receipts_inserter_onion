use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Item{
    pub item_code: String,
    pub price : f32,
    pub quantity : u16
}
