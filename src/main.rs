use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct CurrencyModelList {
    data: Vec<CurrencyModel>,
    timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CurrencyModel {
    id: String,
    name: String,
    symbol: String,
    rank: String,
    price_usd: String, //todo: use decimal type
}

fn http_get(api_url: &str) -> Result<String, Box<dyn Error>> {
    println!("Fetching assets from: '{}'.", api_url);
    let resp = reqwest::blocking::get(api_url)?.text()?;
    // println!("Received response:\n{:#?}", resp);
    Ok(resp)
}

fn main() {
    let api_url = "https://api.coincap.io/v2/assets/";
    let binding = http_get(api_url).unwrap();
    let json = binding.as_str();

    let parsed_currencies = serde_json::from_str::<CurrencyModelList>(json).unwrap();

    println!("Parsed result:\n{:#?}", parsed_currencies);
}
