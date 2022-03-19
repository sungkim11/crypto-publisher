extern crate serde;
use serde::{Serialize, Deserialize};

extern crate reqwest;
use reqwest::Client;

extern crate clap;
use clap::Parser;

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

#[derive(Parser, Debug)]
#[clap(author, version="0.1.0", about="Crypto Price Publisher - Command Line Interface (CLI) Application")]
struct Cli {
    /// Currency Symbol. An example would be BTC.
    #[clap(short, long, default_value = "BTC")]
    currency: String,
    #[clap(short, long, default_value = "USD")]
    /// Rates Symbol. An example would be USD.
    rates: String,
}

pub fn crypto_publisher(){

    let args = Cli::parse();

    let currency = &args.currency;
    let rates = &args.rates;

    let spot_price = get_coin_price("spot".to_string(), currency.to_string(), rates.to_string());
    println!("{}-{} SPOT Price: {:?}", currency.to_string(), rates.to_string(), spot_price.unwrap());
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