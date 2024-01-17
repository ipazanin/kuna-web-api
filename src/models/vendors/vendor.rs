use serde::{Deserialize, Serialize};

use super::vendor_type::VendorType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    id: u32,
    name: String,
    vendor_type: VendorType,
}

impl Vendor {
    pub fn new(id: u32, name: String, vendor_type: VendorType) -> Self {
        Vendor {
            id,
            name,
            vendor_type,
        }
    }
}
