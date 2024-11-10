use std::sync::Mutex;
use actix_web::{App, HttpServer};
use controller::transaction::{add_transaction, delete_transaction, view_transaction};
use db::connection::establish_connection;
use models::tracker::Tracker;
use state::AppState;
use crate::controller::transaction::fetch_financial_data;

mod db;
mod models;
mod controller;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Establishing connection to db
    let db_pool = establish_connection().await;
    println!("DB Pool {:?}", db_pool);

    // Setting up App State to share Tracker across Threads
    let app_state = actix_web::web::Data::new(AppState {
        tracker: Mutex::new(Tracker::new()),
        db_pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/transactions", actix_web::web::post().to(add_transaction))
            .route("/transactions", actix_web::web::get().to(view_transaction))
            .route("/transactions/{id}", actix_web::web::delete().to(delete_transaction))
            .route("/fetch-market-data", actix_web::web::get().to(fetch_financial_data))
    })
    .workers(10)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}