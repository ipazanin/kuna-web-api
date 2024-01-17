use std::time::SystemTime;

use crate::models::{
    categories::{category::Category, label::Label, subcategory::Subcategory},
    items::item::Item,
    shared::metadata::Metadata,
    transactions::{
        location::Location, money::Money, money_unit::MoneyUnit, transaction::Transaction,
    },
    vendors::{vendor::Vendor, vendor_type::VendorType},
};

pub fn get_fake_transactions() -> Vec<Transaction> {
    let food = Category::new(1, "Food".to_owned());
    let drinks = Subcategory::new(1, "Drinks".to_owned(), food);
    let espresso = Item::new(1, "Espresso".to_owned(), drinks);

    let coffee_bar = VendorType::new(1, "Coffee bar".to_owned());
    let peka = Vendor::new(1, "Peka".to_owned(), coffee_bar);

    let work_coffee = Label::new(1, "Work coffe".to_owned());
    let labels = vec![work_coffee];

    let metadata = Metadata::new(SystemTime::now(), SystemTime::now());

    let first_transaction = Transaction::new(
        1,
        metadata,
        Location::new(0.0, 0.0),
        Money::new(false, 10, 0, MoneyUnit::Euro),
        espresso,
        peka,
        labels,
    );

    let transactions = vec![first_transaction];

    transactions
}
