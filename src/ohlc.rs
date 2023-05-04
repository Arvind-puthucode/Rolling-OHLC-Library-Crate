// src/ohlc.rs
pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub open_time: u64,
    pub close_time: u64,
}

impl OHLC {
    pub fn update_ohlc(&mut self, price: f64, timestamp: u64) {
        self.high = self.high.max(price);
        self.low = self.low.min(price);
        self.close = price;
        self.close_time = timestamp;
    }
}
