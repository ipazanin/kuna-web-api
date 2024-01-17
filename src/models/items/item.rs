use serde::{Deserialize, Serialize};

use crate::models::categories::subcategory::Subcategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    name: String,
    subcategory: Subcategory,
}

impl Item {
    pub fn new(id: u32, name: String, subcategory: Subcategory) -> Self {
        Item {
            id,
            name,
            subcategory,
        }
    }
}
