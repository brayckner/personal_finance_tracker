use tracker::Tracker;
use transaction::Transaction;

mod transaction;
mod tracker;
mod ui;
mod utils;

fn main() {
    println!("Personal Finance Tracker");
    let mut tracker = Tracker::new();

    let transaction_one = Transaction::new(001, 70.85, "Georgia Tech Bookstore".to_string(), "09-19-2024".to_string());
    let transaction_two = Transaction::new(002, 100.25, "University of Cincinati Football Game".to_string(), "09-12-2024".to_string());
    let transaction_three = Transaction::new(003, 437.50, "Game Night with the boys".to_string(), "09-10-2024".to_string());

    tracker.add_transaction(&transaction_one);
    tracker.add_transaction(&transaction_two);
    tracker.add_transaction(&transaction_three);

    tracker.view_transactions();

    tracker.delete_transaction(transaction_two.id);
    println!("______________________________________");
    tracker.view_transactions();

}
