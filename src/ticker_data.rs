// src/ticker_data.rs

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TickerData {
    pub b: f64, // Bid price
    pub a: f64, // Ask price
    pub T: u64, // Timestamp
}
