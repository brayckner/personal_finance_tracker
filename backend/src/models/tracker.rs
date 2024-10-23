// Manages a list of transactions, including adding, deleting, and retrieving transactions.
// Implements methods for calculating the balance and generating reports.

use uuid::Uuid;
use super::transaction::Transaction;

pub struct Tracker {
    transactions: Vec<Transaction>
}

impl Tracker {

    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, transaction: &Transaction) {
        self.transactions.push(transaction.clone());
    }

    pub fn delete_transaction(&mut self, transaction_id: Uuid) {
        let transaction_index = self.transactions.iter().position(|x| x.id == transaction_id);

        if let Some(index) = transaction_index {
            self.transactions.remove(index);
        } else {
            println!("Transaction ID: [{}] - Not Found", transaction_id);
        }
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}