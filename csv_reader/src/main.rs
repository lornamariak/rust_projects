use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // get the file name as a command line argument
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let path = Path::new(file_name);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // read the contents of the file 
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Err(why) => panic!("couldn't read line: {}", why),
            Ok(line) => line,
        };
        println!("{}", line);
    }
}

