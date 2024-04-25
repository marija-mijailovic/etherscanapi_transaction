use crate::models::TransactionRequest;
use crate::service;
use actix_web::{web, HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

pub async fn get_block_number() -> impl Responder {
    let block_number = service::get_block_number().await;
    HttpResponse::Ok().body(format!("{:?}", block_number))
}

pub async fn get_transactions(req: web::Json<TransactionRequest>) -> impl Responder {
    let transactions = service::get_transactions(req.into_inner()).await.unwrap();
    HttpResponse::Ok().json(transactions)
}
