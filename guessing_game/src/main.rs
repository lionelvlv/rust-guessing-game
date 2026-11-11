use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    // rust variables are immutable by default
    
    let secret_number = rand::rng().random_range(1..=100);
    // println!("your secret number is {secret_number}");
    
    loop {
        // now for user input
        let mut guess = String::new();
        println!("please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
