mod http_api;
mod logger_config;
mod time_utils;

use log::{error, info};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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

#[inline(always)]
fn parse_int(s: &String) -> i32 {
    s.parse::<i32>()
        .unwrap_or_else(|_| panic!("'{}' is not a number", s))
}

fn get_top_10(currencies: &Vec<CurrencyModel>) -> Vec<String> {
    let mut currencies = currencies.to_owned();
    currencies.sort_by(|a, b| {
        let a_rank = parse_int(&a.rank);
        let b_rank = parse_int(&b.rank);
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
    logger_config::setup_logger();

    let api_url = "https://api.coincap.io/v2/assets/";
    let json = http_api::get(api_url).expect("API error. Failed to fetch data from CoinCap.io");

    let parsed: CurrencyModelList =
        serde_json::from_str::<CurrencyModelList>(json.as_ref()).expect("JSON parsing error");

    let timestamp = parsed.timestamp.to_u64().unwrap_or_else(|| {
        error!("Invalid timestamp {}", parsed.timestamp);
        0
    });
    let timestamp_str = time_utils::get_formatted_local_time(timestamp);
    let all_currencies = parsed.data;
    let top_10_currencies = get_top_10(&all_currencies);

    info!("All currencies: {:#?}", all_currencies);
    info!("Top 10 currencies: {:#?}", top_10_currencies);
    info!("Rates fetched at: {:#?}", timestamp_str);
}
