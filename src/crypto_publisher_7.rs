extern crate serde;
use serde::{Serialize, Deserialize};

extern crate reqwest;
use reqwest::Client;

extern crate clap;
use clap::Parser;

use log::info;
use std::{time::Duration, thread::sleep};

use rdkafka::util::get_rdkafka_version;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::OwnedHeaders;


#[derive(Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice
}

#[derive(Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinbaseTime {
    pub data: CoinTime
}

#[derive(Deserialize, Debug)]
pub struct CoinTime {
    pub iso: String,
    pub epoch: i64,    
}

#[derive(Serialize, Debug)]
pub struct CryptoPriceData {
    pub data: CryptoPrice
}

#[derive(Serialize, Debug)]
pub struct CryptoPrice {
    pub quote_time: String,
    pub currency: String,
    pub rate: String,
    pub spot_price: String,
    pub buy_price: String,
    pub sell_price: String,
    pub spread_price: String,
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
    #[clap(short, long, default_value = "30000")]
    /// Interval. An example would be an interval of 30000 miliseconds or 30 seconds.
    interval: u64,
    #[clap(short, long, default_value = "10")]
    /// Repeat Frequency. An example would be repeat frequency of 10 times.
    frequency: i32,
    #[clap(short, long, default_value = "localhost:9092")]
    /// Broker.
    broker: String,
    #[clap(short, long, default_value = "crypto_price")]
    /// Topic Name.
    topic: String,
}

pub fn crypto_publisher(){

    let args = Cli::parse();
    let mut count = 0i32;

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    loop {
        if count == args.frequency {break;}    

        let currency = &args.currency;
        let rates = &args.rates;
        let quote_time = get_coin_time();
        let broker = &args.broker;
        let topic = &args.topic;
        
        let spot_price = get_coin_price("spot".to_string(), currency.to_string(), rates.to_string());
        let buy_price = get_coin_price("buy".to_string(), currency.to_string(), rates.to_string());
        let sell_price = get_coin_price("sell".to_string(), currency.to_string(), rates.to_string());
        
        let quote_time = quote_time.as_ref();
        let spot_price = spot_price.as_ref();
        let buy_price = buy_price.as_ref();
        let sell_price = sell_price.as_ref();
      
        let spread_price: f32 = (buy_price.unwrap().parse::<f32>().unwrap()) - (&sell_price.unwrap().parse::<f32>().unwrap());

        let price_screen = format!("{}: {}-{} SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}", quote_time.unwrap(), currency.to_string(), rates.to_string(), spot_price.unwrap(), buy_price.unwrap(), sell_price.unwrap(), spread_price.to_string());

        println!("{}", price_screen);

        let price_struct = CryptoPriceData {
            data: CryptoPrice {
                quote_time: quote_time.unwrap().to_string(),
                currency: currency.to_string(),
                rate: rates.to_string(),
                spot_price: spot_price.unwrap().to_string(),
                buy_price: buy_price.unwrap().to_string(),
                sell_price: sell_price.unwrap().to_string(),
                spread_price: spread_price.to_string(),
            }
        };
        
        let price_json = serde_json::to_string(&price_struct).unwrap();
        publish(broker, topic, &price_json, count);

        sleep(Duration::from_millis(args.interval));
        count += 1;         
    }
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

#[tokio::main]
async fn get_coin_time() -> Result<std::string::String, Box<dyn std::error::Error>> {    

    let request_url = format!("https://api.coinbase.com/v2/time");

    let client = Client::new();
    let resp_time = client.get(&request_url).send().await?.json::<CoinbaseTime>().await?;
 
    Ok(resp_time.data.iso)
 }

#[tokio::main]
async fn publish(broker: &str, topic: &str, pub_message: &str, count: i32) {
    
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("message.timeout.ms", "5000")
        .set("security.protocol", "plaintext")
        .create()
        .expect("Failed to create producer");

        let payload = format!("message {}", pub_message);
        let key = format!("key {}", count);

        info!("Sending message '{}'", count);

        let status = producer.send(
            FutureRecord::to(topic)
                .payload(&payload)
                .key(&key)
                .headers(OwnedHeaders::new().add(
                    &format!("header_key_{}", count),
                    &format!("header_value_{}", count)
                )),
            Duration::from_secs(0)
        ).await;

        info!("Status '{:?}' received from message '{}'", status, count);
}
