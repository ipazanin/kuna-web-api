use axum::{http::StatusCode, Json};

use crate::{
    data::in_memory_data::fake_data::get_fake_transactions,
    models::transactions::transaction::Transaction,
};

pub async fn get_all_transactions() -> (StatusCode, Json<Vec<Transaction>>) {
    let transactions = get_fake_transactions();

    (StatusCode::OK, Json(transactions))
}
