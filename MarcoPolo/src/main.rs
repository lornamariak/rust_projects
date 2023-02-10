use std::io;

fn main() {
    println!("Welcome to the Marco Polo game!");
    println!("The rules are simple: one player says 'Marco', and the other player responds with 'Polo'");

    loop {
        println!("Enter 'Marco' to start the game:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();

        if input == "marco" {
            println!("Polo!");
        } else {
            println!("That's not how the game is played. Please try again.");
        }
    }
}