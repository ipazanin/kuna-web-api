use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    id: u32,
    name: String,
}

impl Category {
    pub fn new(id: u32, name: String) -> Self {
        Category { id, name }
    }
}
