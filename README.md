# Rolling OHLC Library Crate
This Rust library crate provides a convenient way of computing rolling OHLC (open-high-low-close) values for a stream of numeric prices and timestamps, based on a given time window.

## Features
* Rolling OHLC Calculation: Calculate the rolling OHLC values for a given time window based on a stream of prices and timestamps.
* Customizable Time Window: Set the desired time window for calculating the rolling OHLC values.
* Efficient Computation: The library crate is designed to efficiently process large datasets and provide accurate results in a timely manner.

##Installation
* To use this crate in your Rust project, add the following line to your Cargo.toml file:
 * `[dependencies]
  rolling_ohlc = "0.1.0" `

## Usage

``` 
use rolling_ohlc::{RollingOHLC, PriceTick};

fn main() {
    // Create a new RollingOHLC instance with a 5-minute time window
    let mut rolling_ohlc = RollingOHLC::new(5 * 60); // 5 minutes in seconds

    // Process price ticks and compute rolling OHLC
    let price_tick1 = PriceTick::new(1620732900, 10.5); // Timestamp: 1620732900, Price: 10.5
    rolling_ohlc.process_tick(price_tick1);

    let price_tick2 = PriceTick::new(1620732960, 11.2); // Timestamp: 1620732960, Price: 11.2
    rolling_ohlc.process_tick(price_tick2);

    // Get the current rolling OHLC values
    let ohlc = rolling_ohlc.get_current_ohlc();
    println!("Open: {}", ohlc.open);
    println!("High: {}", ohlc.high);
    println!("Low: {}", ohlc.low);
    println!("Close: {}", ohlc.close);
} 
```
## Example
  You can find an example implementation using this crate to read JSON data and calculate rolling OHLC for multiple symbols in the main.rs file.

## Contribution
  Contributions are welcome! If you have any ideas, improvements, or bug fixes, please open an issue or submit a pull request on GitHub.


