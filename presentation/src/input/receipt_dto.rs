use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::body::{BodySize, MessageBody};
use actix_web::web::Bytes;
use domain::{Receipt, Item};
use serde::{Serialize, Deserialize};
use crate::input::item_dto::ItemDto;

#[derive(Serialize, Deserialize)]
pub struct ReceiptDto {
    id : String,
    items : Vec<ItemDto>
}

impl From<ReceiptDto> for Receipt {
    fn from(value: ReceiptDto) -> Self {
        Receipt {
            id: value.id,
            items: value.items.into_iter().map(|item| Item::from(item)).collect()
        }
    }
}

impl From<Receipt> for ReceiptDto {
    fn from(value: Receipt) -> Self {
        ReceiptDto {
            id: value.id,
            items: value.items.into_iter().map(|item| ItemDto::from(item)).collect()
        }
    }
}
