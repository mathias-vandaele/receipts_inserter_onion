use domain::Item;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ItemDto{
    item_code: String,
    price : f32,
    quantity : u16
}

impl Into<ItemDto> for Item {
    fn into(self) -> ItemDto {
        ItemDto {
            item_code : self.item_code,
            price : self.price,
            quantity : self.quantity
        }
    }
}