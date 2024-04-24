use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub etherscan_url: String,
    pub etherscan_api_key: String,
    pub contract_address: String,
    pub app_url: String,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv().ok();
        let etherscan_url = env::var("ETHERSCAN_URL").expect("ETHERSCAN_URL must be set");
        let etherscan_api_key =
            env::var("ETHERSCAN_API_KEY").expect("ETHERSCAN_API_KEY must be set");
        let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");
        let app_url = env::var("APP_URL").expect("APP_URL must be set");

        Self {
            etherscan_url,
            etherscan_api_key,
            contract_address,
            app_url,
        }
    }
}
