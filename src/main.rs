//GET COIN PRICE
extern crate serde_derive;

use {reqwest, std::io, tokio};

const HOST_ROOT: &str = "https://api.coingecko.com/api/v3/simple/price?";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .send()
        .await
        .expect("Failed to get response!")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn blockchain_price_request(coin: &str) {
    let response = send_request(&[HOST_ROOT, "ids=", &coin, "&vs_currencies=usd"].join(""));
    println!("{}", response);
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

enum Manager {
    BTC,
    ETH,
    SOL,
}

impl Manager {
    fn show() {
        println!("== Please choose coin to get price ==");
        println!("");
        println!("1. Bitcoin");
        println!("2. Ethereum");
        println!("3. Solana");
        println!("");
        println!("======================================")
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::BTC),
            "2" => Some(Manager::ETH),
            "3" => Some(Manager::SOL),
            _ => None,
        }
    }
}

fn main() {
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");
        match Manager::choice(&input.as_str()) {
            Some(Manager::BTC) => blockchain_price_request("bitcoin"),
            Some(Manager::ETH) => blockchain_price_request("ethereum"),
            Some(Manager::SOL) => blockchain_price_request("solana"),
            None => break,
        }
    }
}
