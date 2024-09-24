use tracker::Tracker;
use transaction::Transaction;
use ui::{display_menu, get_ammount, get_category, get_date, get_user_selection, user_set_id};

mod tracker;
mod transaction;
mod ui;
mod utils;

fn main() {
    let mut tracker = Tracker::new();

    loop {
        display_menu();
        let user_selection = get_user_selection();

        if Some(user_selection).is_some() {
            if user_selection == 1 {
                let id = tracker.set_initial_id();
                let amount = get_ammount();
                let category = get_category();
                let date = get_date();
                let transaction = Transaction::new(id, amount, category, date);
                tracker.add_transaction(&transaction);
            }

            if user_selection == 2 {
                tracker.view_transactions();
            }

            if user_selection == 3 {
                let transaction_id = user_set_id();
                tracker.delete_transaction(transaction_id);
            }

            if user_selection == 4 {
                println!("Doing operation: {}", user_selection);
            }

            if user_selection == 5 {
                println!("Exiting Program. Bye.");
                break;
            }
        }
    }
}
