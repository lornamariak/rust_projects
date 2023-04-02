use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use csv::{ReaderBuilder, StringRecord};
use itertools::Itertools;
use statrs::statistics::{Mean, Median, Mode, Max, Min};

fn main() -> Result<(), Box<dyn Error>> {
    // Prompt for the CSV file name and column to analyze
    let file_name = String::from("data.csv");
    let column_name = String::from("price");

    // Load file into memory
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse file(csv)
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(contents.as_bytes());
    let header = reader.headers()?.clone();

    // Find index of the selected column
    let column_index = header.iter().position(|c| c == column_name).unwrap();

    // Extract the values in the selected column
    let mut column_values = vec![];
    for result in reader.records() {
        let record = result?;
        column_values.push(record.get(column_index).unwrap().parse::<f64>().unwrap());
    }

    // Compute summary statistics
    let mean = Mean::new(&column_values).unwrap();
    let median = Median::new(&column_values).unwrap();
    let mode = Mode::new(&column_values).unwrap().next().unwrap();
    let min = Min::new(&column_values).unwrap();
    let max = Max::new(&column_values).unwrap();

    // Display summary statistics
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
    println!("Minimum: {}", min);
    println!("Maximum: {}", max);

    Ok(())
}
