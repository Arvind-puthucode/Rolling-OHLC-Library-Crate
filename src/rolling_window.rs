use std::collections::VecDeque;

pub struct RollingWindow {
    window_size: u64,
    data_points: VecDeque<(f64, u64)>,
}

impl RollingWindow {
    pub fn new(window_size: u64) -> Self {
        RollingWindow {
            window_size,
            data_points: VecDeque::new(),
        }
    }

    pub fn add_data(&mut self, price: f64, timestamp: u64) {
        self.data_points.push_back((price, timestamp));
        while self.data_points.front().unwrap().1 < timestamp - self.window_size {
            self.data_points.pop_front();
        }
    }
    pub fn compute_ohlc(&self) -> OHLC {
        let open = self.data_points.front().unwrap().0;
        let close = self.data_points.back().unwrap().0;
        let high = self.data_points
            .iter()
            .map(|(price, _)| *price)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let low = self.data_points
            .iter()
            .map(|(price, _)| *price)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        OHLC { open, high, low, close }
    }

    
}

pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}
