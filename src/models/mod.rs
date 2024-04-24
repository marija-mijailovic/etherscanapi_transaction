use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub address: String,
    pub years_in_past: i64,
}
