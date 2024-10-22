// Represents a financial transaction with fields like id, amount, category, and date.
// Methods for creating, displaying, and serializing transactions.

use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct TransactionInput {
    pub amount: f64,
    pub category: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: u32,
    pub amount: f64,
    pub category: String,
    pub date: String,
}
#[derive(Clone, Serialize)]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
    pub category: String,
    pub date: String,
}

impl Transaction {

    pub fn new(id: u32, amount: f64, category: String, date: String) -> Self {
        Self {id, amount, category, date}
    }
}

