use std::io;

pub fn display_menu() {
    println!("\n--------- Personal Finance Tracker ---------");
    println!("1. Add Transaction");
    println!("2. View Transactions");
    println!("3. Delete Transaction");
    println!("4. Summary Report");
    println!("5. Exit");
    println!("--------------------------------------------\n");
}

pub fn get_user_selection() -> u32 {
    println!("Please Enter a selection to continue: ");
    let mut user_selection = String::new();
    io::stdin().read_line(&mut user_selection).expect("Failed to read user input.");
    let user_selection: u32 = user_selection.trim().parse().expect("Please input a number");
    user_selection
}

pub fn get_ammount() -> f64 {
    println!("Enter an amount for the transaction: $");
    let mut ammount = String::new();
    io::stdin().read_line(&mut ammount).expect("Failed to read transaction ammount.");
    let ammount: f64 = ammount.trim().parse().expect("Please input a valid transaction ammount."); 
    ammount
}

pub fn get_category() -> String {
    println!("Enter a category tag name for this transaction: ");
    let mut category = String::new();
    io::stdin().read_line(&mut category).expect("Failed to get category tag name");
    let category: String = category.trim().parse().expect("Please input a valid category tag name.");
    category
}

pub fn get_date() -> String {
    println!("Enter a date in MM/DD/YYYY format: ");
    let mut user_input_date = String::new();
    io::stdin().read_line(&mut user_input_date).expect("Failed to get input date.");
    let user_input_date: String = user_input_date.trim().parse().expect("Please input a valid date.");
    user_input_date
}

pub fn user_set_id() -> u32 {
    println!("Enter the id of the transaction you would like to select: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Faild to get id.");
    let id: u32 = id.trim().parse().expect("Please input a valid id");
    id
}