// Represents a financial transaction with fields like id, amount, category, and date.
// Methods for creating, displaying, and serializing transactions.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Deserialize)]
pub struct TransactionInput {
    pub amount: f64,
    pub category: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub amount: f64,
    pub category: String,
    pub date: String,
}
#[derive(Clone, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: f64,
    pub category: String,
    pub date: String,
}

impl Transaction {

    pub fn new(amount: f64, category: String, date: String) -> Self {
        Self {id: Uuid::new_v4(), amount, category, date}
    }
}

