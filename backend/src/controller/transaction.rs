use actix_web::{ HttpResponse, Responder};
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