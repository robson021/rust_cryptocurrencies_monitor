use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct ApiResponse {
    id: String,
    name: String,
    symbol: String,
    rank: String,
    price_usd: String, //todo: use decimal type
}

fn http_get(api_url: &str) -> Result<String, Box<dyn Error>> {
    println!("Fetching assets from: '{}'.", api_url);
    let resp = reqwest::blocking::get(api_url)?.text()?;
    println!("Received response:\n{:#?}", resp);
    Ok(resp)
}

fn main() {
    let api_url = "https://api.coincap.io/v2/assets/";
    let result = http_get(api_url).unwrap();

    println!("Result:\n{}", result);
}
