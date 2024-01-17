use serde::{Deserialize, Serialize};

use crate::models::{
    categories::label::Label, items::item::Item, shared::metadata::Metadata,
    vendors::vendor::Vendor,
};

use super::{location::Location, money::Money};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    id: u32,
    metadata: Metadata,
    location: Location,
    amount: Money,
    item: Item,
    vendor: Vendor,
    labels: Vec<Label>,
}

impl Transaction {
    pub fn new(
        id: u32,
        metadata: Metadata,
        location: Location,
        amount: Money,
        item: Item,
        vendor: Vendor,
        labels: Vec<Label>,
    ) -> Self {
        Transaction {
            id,
            metadata,
            location,
            amount,
            item,
            vendor,
            labels,
        }
    }
}
