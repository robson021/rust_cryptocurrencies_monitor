use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct CurrencyModelList {
    data: Vec<CurrencyModel>,
    timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CurrencyModel {
    id: String,
    name: String,
    symbol: String,
    rank: String,
    price_usd: Decimal,
}

fn http_get(api_url: &str) -> Result<String, Box<dyn Error>> {
    println!("Fetching assets from: '{}'.", api_url);
    let resp = reqwest::blocking::get(api_url)?.text()?;
    // println!("Received response:\n{:#?}", resp);
    Ok(resp)
}

fn get_top_10(currencies: &Vec<CurrencyModel>) -> Vec<String> {
    let mut currencies = currencies.to_owned();
    currencies.sort_by(|a, b| {
        let a_rank = a.rank.parse::<i32>().unwrap();
        let b_rank = b.rank.parse::<i32>().unwrap();
        a_rank.cmp(&b_rank)
    });
    currencies
        .iter()
        .map(|c| {
            let name = c.name.as_str();
            let price = c.price_usd.round_dp(2).trunc_with_scale(2);
            format!("{name} ({price}$)")
        })
        .take(10)
        .collect::<Vec<String>>()
}

fn main() {
    let api_url = "https://api.coincap.io/v2/assets/";
    let json = http_get(api_url).unwrap();

    let all_currencies = serde_json::from_str::<CurrencyModelList>(json.as_ref())
        .unwrap()
        .data;

    let top_10_currencies = get_top_10(&all_currencies);
    println!("All currencies: {:#?}", all_currencies);
    println!("Top 10 currencies: {:#?}", top_10_currencies);
}
