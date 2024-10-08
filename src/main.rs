use std::sync::Mutex;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use tracker::Tracker;
use transaction::Transaction;

mod tracker;
mod transaction;
mod ui;
mod utils;

struct AppState {
    tracker: Mutex<Tracker>,
}

#[derive(Deserialize)]
struct TransactionInput {
    amount: f64,
    category: String,
    date: String,
}

#[derive(Serialize)]
struct TransactionResponse {
    id: u32,
    amount: f64,
    category: String,
    date: String,
}

async fn add_transaction(data: actix_web::web::Data<AppState>, transaction: actix_web::web::Json<TransactionInput>) -> impl Responder {
    let mut tracker = data.tracker.lock().unwrap();
    let id = tracker.set_initial_id();
    let new_transaction = Transaction::new(id, transaction.amount, transaction.category.clone(), transaction.date.clone());
    tracker.add_transaction(&new_transaction);

    HttpResponse::Ok().json(TransactionResponse {
        id, 
        amount: transaction.amount, 
        category: transaction.category.clone(), 
        date: transaction.date.clone(),
    })
}

async fn view_transaction(data: actix_web::web::Data<AppState>) -> impl Responder {
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

    HttpResponse::Ok().json(transactions)
}

async fn delete_transaction(data: actix_web::web::Data<AppState>, id: actix_web::web::Path<u32>) -> impl Responder {
    let mut tracker = data.tracker.lock().unwrap();
    tracker.delete_transaction(*id);
    HttpResponse::Ok().body("Transaction Deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Setting up App State to share Tracker across Threads
    let app_state = actix_web::web::Data::new(AppState {
        tracker: Mutex::new(Tracker::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/transactions", actix_web::web::post().to(add_transaction))
            .route("/transactions", actix_web::web::get().to(view_transaction))
            .route("/transactions/{id}", actix_web::web::delete().to(delete_transaction))
    })
    .workers(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}