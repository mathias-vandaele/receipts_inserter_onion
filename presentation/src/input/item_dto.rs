use actix_web::body::MessageBody;
use domain::Item;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ItemDto{
    item_code: String,
    price : f32,
    quantity : u16
}

impl From<ItemDto> for Item {
    fn from(value: ItemDto) -> Self {
        Item {
            item_code: value.item_code,
            price: value.price,
            quantity: value.quantity
        }
    }
}

impl From<Item> for ItemDto {
    fn from(value: Item) -> Self {
        ItemDto {
            item_code: value.item_code,
            price: value.price,
            quantity: value.quantity
        }
    }
}


