use serde::{Deserialize, Serialize};

use super::money_unit::MoneyUnit;

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    is_negative: bool,
    value: u32,
    decimal_value: u32,
    unit: MoneyUnit,
}

impl Money {
    pub fn new(is_negative: bool, value: u32, decimal_value: u32, unit: MoneyUnit) -> Self {
        Money {
            is_negative,
            value,
            decimal_value,
            unit,
        }
    }
}
