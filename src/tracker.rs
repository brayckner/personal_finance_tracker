// Manages a list of transactions, including adding, deleting, and retrieving transactions.
// Implements methods for calculating the balance and generating reports.

use crate::transaction::Transaction;

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

    pub fn delete_transaction(&mut self, transaction_id: u32) {
        let transaction_index = self.transactions.iter().position(|x| x.id == transaction_id);

        if let Some(index) = transaction_index {
            self.transactions.remove(index);
        } else {
            println!("Transaction ID: [{}] - Not Found", transaction_id);
        }
    }

    // TODO: This needs to be replaced with just getting a random number uuid. 
    pub fn set_initial_id(&self) -> u32 {
        self.transactions.len().try_into().unwrap()
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}