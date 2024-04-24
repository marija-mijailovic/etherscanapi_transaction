use crate::config::AppConfig;
use crate::models::TransactionRequest;
use chrono::{Duration, Utc};
use reqwest::Error;
use serde_json::Value;

pub async fn get_block_number() -> Result<u64, Error> {
    let url = format!(
        "{}?module=proxy&action=eth_blockNumber&apikey={}",
        AppConfig::init().etherscan_url,
        AppConfig::init().etherscan_api_key
    );
    let response: Value = reqwest::get(url).await?.json().await?;
    let block_number_hex = response["result"].as_str().unwrap();
    let block_number = u64::from_str_radix(&block_number_hex[2..], 16).unwrap();

    Ok(block_number)
}

pub async fn get_transactions(req: TransactionRequest) -> Result<Value, Error> {
    let url = format!(
        "{}?module=account&action=tokentx&contractaddress={}&address={}&startblock=0&endblock=999999999&apikey={}", 
        AppConfig::init().etherscan_url,
        AppConfig::init().contract_address,
        req.address,
        AppConfig::init().etherscan_api_key);
    let mut response: Value = reqwest::get(&url).await?.json().await?;

    let in_past_timestamp = (Utc::now() - Duration::days(req.years_in_past * 365)).timestamp();

    if let Some(transactions) = response["result"].as_array_mut() {
        transactions.retain(|transaction| {
            let timestamp = transaction["timeStamp"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            timestamp >= in_past_timestamp
        });
    }

    Ok(response)
}
