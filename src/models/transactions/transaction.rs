use std::time::SystemTime;

use super::{location::Location, money::Money};

#[derive(Debug)]
pub struct Transaction {
    id: u32,
    created_at: SystemTime,
    updated_at: SystemTime,
    location: Location,
    amount: Money,
}
