use transaction::Transaction;

mod transaction;
mod tracker;
mod ui;
mod utils;

fn main() {
    println!("Personal Finance Tracker");

    let transaction = Transaction::new(111, 70.85, "Georgia Tech Bookstore".to_string(), "09-19-2024".to_string());
    transaction.display()
    
}
