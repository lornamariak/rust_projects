use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();

    // Get data generation params
    let num_records = get_user_input("Records to generate: ").parse::<usize>().unwrap();
    let data_range = get_user_input("Data range (e.g. 1-100): ");
    let data_format = get_user_input("Data format (int, float, or string): ");

    // Parse data ranges
    let range_parts: Vec<&str> = data_range.split('-').collect();
    let min_value = range_parts[0].parse::<f64>().unwrap();
    let max_value = range_parts[1].parse::<f64>().unwrap();

    // Generate and print data records
    for _ in 0..num_records {
        match data_format.as_str() {
            "int" => {
                let data = rng.gen_range(min_value as i32, max_value as i32 + 1);
                println!("{}", data);
            }
            "float" => {
                let data = rng.gen_range(min_value, max_value);
                println!("{:.2}", data);
            }
            "string" => {
                let data = generate_random_string(rng.gen_range(min_value as usize, max_value as usize + 1));
                println!("{}", data);
            }
            _ => {
                eprintln!("Invalid data format specified.");
                std::process::exit(1);
            }
        }
    }
}

// function to get user input
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_owned()
}

// function to generate a random string
fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let result: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    result
}
