use log::{debug, error, info};
use std::error::Error;

pub fn get(api_url: &str) -> Result<String, Box<dyn Error>> {
    info!("GET: {}", api_url);
    let resp = reqwest::blocking::get(api_url)
        .map_err(|e| error!("Failed to get from: {}", e))
        .unwrap()
        .text()?;
    debug!("Received response: {:#?}", resp);
    Ok(resp)
}
