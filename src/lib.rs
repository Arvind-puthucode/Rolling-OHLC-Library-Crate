mod input_parser;
mod data_structure;
mod ohlc;
mod moving_window;

pub use input_parser::InputStruct;
pub use moving_window::Window;
pub use moving_window::read_time_stamps;