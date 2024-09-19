// Represents a financial transaction with fields like id, amount, category, and date.
// Methods for creating, displaying, and serializing transactions.

#[derive(Clone)]
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

    pub fn display(&self) {
        println!("ID: {}, Amount: {}, Category: {}, Date: {}", self.id, self.amount, self.category, self.date);
    }
    
}

