use super::vendor_type::VendorType;

#[derive(Debug)]
pub struct Vendor {
    id: u32,
    name: String,
    vendor_type: VendorType,
}