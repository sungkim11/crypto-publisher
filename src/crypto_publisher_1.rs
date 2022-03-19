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

#[tokio::main]
pub async fn crypto_publisher() -> Result<(), Box<dyn std::error::Error>> {

    let spot_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/spot",
        currency = "BTC",
        rates = "USD");

    let resp_spot_price = Client::new().get(&spot_url).send().await?.json::<CoinbasePrice>().await?;

    println!("SPOT: {base}-{currency}: {amount}",
        base=resp_spot_price.data.base,
        currency=resp_spot_price.data.currency,
        amount=resp_spot_price.data.amount);

    Ok(())
}

