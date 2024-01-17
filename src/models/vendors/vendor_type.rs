use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VendorType {
    id: u32,
    name: String,
}

impl VendorType {
    pub fn new(id: u32, name: String) -> Self {
        VendorType { id, name }
    }
}
