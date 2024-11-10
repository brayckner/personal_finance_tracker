use std::env;
use actix_web::{HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::transaction::TransactionInput;
use crate::services::transaction::{create_transaction, fetch_transactions, remove_transaction};
use crate::state::AppState;

pub async fn add_transaction(data: actix_web::web::Data<AppState>, transaction: actix_web::web::Json<TransactionInput>) -> impl Responder {
    return match create_transaction(&data, transaction.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn view_transaction(data: actix_web::web::Data<AppState>) -> impl Responder {
    return match fetch_transactions(&data).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_transaction(data: actix_web::web::Data<AppState>, id: actix_web::web::Path<Uuid>) -> impl Responder {
    return match remove_transaction(&data, *id).await {
        Ok(_) => HttpResponse::Ok().body("Transaction Deleted"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
pub struct TiingoApiResponse {
    pub message: String,
}
pub async fn fetch_financial_data() -> impl Responder {

    // Creating HTTP Client
    let client = Client::new();

    // API URL and Token
    let url = "https://api.tiingo.com/api/test/";
    let auth_token = env::var("TIINGO_API_KEY").expect("TIINGO_API_KEY not set in .env");

    // prepare the request with headers
    let response = client.get(url)
        .header("Content-type", "application/json")
        .header("Authorization", format!("Token {}", auth_token))
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<TiingoApiResponse>().await {
                    Ok(api_response) => HttpResponse::Ok().json(api_response),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to parse response")
                }
            } else {
                HttpResponse::InternalServerError().body("External API request failed")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to send request")
    }
}