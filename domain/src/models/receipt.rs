use crate::models::item::Item;

pub struct Receipt {
    pub id : String,
    pub items : Vec<Item>
}
