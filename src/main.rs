use anyhow::Result;
use task1::{read_time_stamps};
use std::env;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: Expected exactly two arguments.");
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    read_time_stamps(input_path, output_path)?;

    Ok(())
}


