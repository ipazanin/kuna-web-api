use serde::{Deserialize, Serialize};

use super::category::Category;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subcategory {
    id: u32,
    name: String,
    category: Category,
}

impl Subcategory {
    pub fn new(id: u32, name: String, category: Category) -> Self {
        Subcategory { id, name, category }
    }
}
