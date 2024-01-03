use crate::models::categories::subcategory::Subcategory;

#[derive(Debug)]
pub struct Item {
    id: u32,
    name: String,
    subcategory: Subcategory,
}
