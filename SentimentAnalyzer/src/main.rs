use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
// Sentiment Rs crate
use sentiment::{analyze, Sentiment};
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: sentiment test");
        std::process::exit(1);
    }
    //read in the filename
    let filename = &args[1];
    //read in the file
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    //go over it line by line
    for line in reader.lines() {
        let text = line.expect("Failed to read line.");
        let score = analyze(text);

    //analyze and compare positive, negative and neutral sentiments for each sentence
        if score.positive > score.negative {
            println!("The text is positive: {}", text);
        } else if score.positive < score.negative {
            println!("The text is negative: {}", text);
        } else {
            println!("The text is neutral: {}", text);
        }
    }
}
