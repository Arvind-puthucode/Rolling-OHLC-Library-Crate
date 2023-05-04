use crate::input_parser::InputStruct;
use crate::ohlc::Ohlc;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::data_structure::MultiSet;
use std::collections::{HashMap, VecDeque};
use anyhow::Result;

pub struct Window {
    time: i64,
    current_window: HashMap<String, VecDeque<WindowContent>>,
    price_heap: HashMap<String, MultiSet<i32>>,
}

impl Window {
    pub fn new(window_length: i64) -> Self {
        Window {
            time: window_length,
            current_window: HashMap::new(),
            price_heap: HashMap::new(),
        }
    }

    pub fn add(&mut self, event: InputStruct) -> Ohlc {
        let event = WindowContent::from(event);
        let price_heap = self
            .price_heap
            .entry(event.symbol.clone())
            .or_insert(MultiSet::new());
        let current_window = self
            .current_window
            .entry(event.symbol.clone())
            .or_insert(VecDeque::new());

        //Add the event to the queue
        current_window.push_back(event.clone());
        price_heap.insert(event.price);

        //Remove the events from queue and its corresponding prices which are outside the window time frame
        while current_window.front().unwrap().timestamp <= event.timestamp - self.time {
            println!("Removing event: {:?}", current_window.front().unwrap());
            let event = current_window.pop_front().unwrap();
            price_heap.remove(&event.price);
        }

        //Calculate the ohlc
        Ohlc {
            symbol: event.symbol,
            timestamp: event.timestamp,
            open: current_window.front().unwrap().price as f64 / 100000000.0,
            high: *price_heap.iter().next_back().unwrap() as f64 / 100000000.0,
            low: *price_heap.iter().next().unwrap() as f64 / 100000000.0,
            close: current_window.back().unwrap().price as f64 / 100000000.0,
        }
    }
}

//Event with only necessary stuff for now
#[derive(Debug, Clone)]
struct WindowContent {
    symbol: String,
    timestamp: i64,
    price: i32,
}

impl From<InputStruct> for WindowContent {
    fn from(input_event: InputStruct) -> Self {
        WindowContent {
            symbol: input_event.s,
            timestamp: input_event.T,
            price: {
                //A workaround for the fact that f64 does not implement ord trait
                let mut price = (input_event.a + input_event.b) / 2.0;
                price *= 100000000.0;
                price as i32
            },
        }
    }
}
pub fn read_time_stamps(input_path: &str, output_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let input_reader = BufReader::new(input_file);
    let mut output_file = File::create(output_path)?;

    let mut window = Window::new(300000);

    for line in input_reader.lines() {
        let line = line?;
        let input_event: InputStruct = line.parse::<InputStruct>()?;

        let output = window.add(input_event);
        let output = serde_json::to_string(&output)?;
        writeln!(output_file, "{}", output)?;
    }

    Ok(())
}