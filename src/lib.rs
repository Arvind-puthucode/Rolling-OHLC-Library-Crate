// src/lib.rs
mod ticker_data;
mod rolling_window;

pub use ticker_data::TickerData;
pub use rolling_window::{RollingWindow, OHLC};

use serde_json;
use tokio::fs::File;
use tokio::io::{self, BufReader, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

// use tokio::prelude::*; // Remove this line

pub fn parse_ticker_data(json_data: &str) -> Result<TickerData, serde_json::Error> {
    serde_json::from_str(json_data)
}

pub async fn process_dataset_file(file_path: &str,output_file_path: &str) -> io::Result<()> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let window_size = 5 * 60 * 1000; // 5 minutes in milliseconds
    let mut rolling_window = RollingWindow::new(window_size);
    let mut output_file = File::create(output_file_path).await?;

    while let Some(line) = lines.next_line().await? {
        let ticker_data = parse_ticker_data(&line).unwrap();
        let price = (ticker_data.b + ticker_data.a) / 2.0; // Use the average of bid and ask prices
        rolling_window.add_data(price, ticker_data.T);

        let ohlc = rolling_window.compute_ohlc();
        // Use the computed OHLC data as needed
        let ohlc_str = format!("O: {:.2}, H: {:.2}, L: {:.2}, C: {:.2}\n", ohlc.open, ohlc.high, ohlc.low, ohlc.close);
        output_file.write_all(ohlc_str.as_bytes()).await?;
    }

    Ok(())
}

