use core::panic;
use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        // println!("The secret number is: {secret_number}");
        //
        // println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // We can do guess again because it's a Rust thing, it's called shadowing, instead of creating
        // two variables, one string, one integer
        // u32 = unsigned 32 bit integer, here we are telling guess to be that, trim the spaces beginning
        // and end as well as /n before parsing the string in "guess" into a 32 bit integer
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // parse returns Result and Result is an enum that has Ok or Err
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
