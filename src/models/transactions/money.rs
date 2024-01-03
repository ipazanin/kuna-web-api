use super::money_unit::MoneyUnit;

#[derive(Debug)]
pub struct Money{
    value: i32,
    decimal_value: i32,
    unit: MoneyUnit,
}