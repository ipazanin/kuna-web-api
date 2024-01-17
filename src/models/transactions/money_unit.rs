use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MoneyUnit {
    Euro,
    UnitedStatesDollar,
}
