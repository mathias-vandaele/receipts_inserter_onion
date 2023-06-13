use domain::{Receipt, Item};
use serde::{Serialize, Deserialize};
use crate::input::item_dto::ItemDto;

#[derive(Serialize, Deserialize)]
pub struct ReceiptDto {
    id : String,
    items : Vec<ItemDto>
}

impl Into<Receipt> for ReceiptDto {
    fn into(self) -> Receipt {
        let mut items : Vec<Item> = Vec::new();
        for item_dto in self.items {
            items.push(item_dto.into());
        }
        Receipt {
            id : self.id,
            items : items
        }
    }
}

