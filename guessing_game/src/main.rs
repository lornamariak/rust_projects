// testing 1 2 

//declare io scope by importing io from std
use std::io;
use rand::Rng;

//fn declares new function, (parameters if any), main function is entry into program, {function body}
fn main() {

    //set the secret number
    let secret_num = rand::thread_rng().gen_range(1..=10);
    
    //println! is a macro
    println!("Guess the number!");
    println!("Please input your guess.");

    //let - used to create a new variable, let mut x = 5 means x is mutable , let x = 5 means is immutable 
    // string is string data type and ::new is a function associated to string
    let mut guess = String::new();

    // call stdin in io lib to handle user input
    io::stdin()
        // call readline method on the standard input handle to get user input
        // & shows that the argument is a reference which allows different parts of the cose access one variable without copy
        .read_line(&mut guess)
        // error handling
        .expect("Failed to read line");
    // println! can read an enumaration placed in {}
    println!("The secret number is {secret_num}");
    println!("You guessed: {guess}");
}