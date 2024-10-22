use uuid::Uuid;

use crate::{models::transaction::{Transaction, TransactionInput, TransactionResponse}, state::AppState};

pub async fn create_transaction(data: &AppState, transaction: TransactionInput) -> Result<TransactionResponse, String> {
    let mut tracker = data.tracker.lock().unwrap();
    let new_transaction = Transaction::new(transaction.amount, transaction.category.clone(), transaction.date.clone());
    tracker.add_transaction(&new_transaction);

    return Ok(TransactionResponse {
        id: new_transaction.id, 
        amount: transaction.amount, 
        category: transaction.category.clone(), 
        date: transaction.date.clone(),
    })
}

pub async fn fetch_transactions(data: &AppState) -> Result<Vec<TransactionResponse>, String> {
    let tracker = data.tracker.lock().unwrap();
    let transactions: Vec<TransactionResponse> = tracker.get_transactions()
        .iter()
        .map(|t| TransactionResponse {
            id: t.id,
            amount: t.amount,
            category: t.category.clone(),
            date: t.date.clone(), 
        })
        .collect();

    return Ok(transactions)
}

pub async fn remove_transaction(data: &AppState, id: Uuid) -> Result<(), String> {
    let mut tracker = data.tracker.lock().unwrap();
    tracker.delete_transaction(id);
    return Ok(())
}