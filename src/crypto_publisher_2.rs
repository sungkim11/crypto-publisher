extern crate serde;
use serde::{Serialize, Deserialize};

extern crate reqwest;
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

pub fn crypto_publisher(){

    let spot_price = get_coin_price("spot".to_string(), "BTC".to_string(), "USD".to_string());
    println!("BTC-USD SPOT Price: {:?}", spot_price.unwrap());
}

#[tokio::main]
async fn get_coin_price(request_type: String, request_currency: String, request_rates: String) -> Result<std::string::String, Box<dyn std::error::Error>> {

    let request_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/{type}",
        currency = request_currency,
        rates = request_rates,
        type = request_type);

    let client = Client::new();
    let resp_price = client.get(&request_url).send().await?.json::<CoinbasePrice>().await?;
 
    Ok(resp_price.data.amount)
}