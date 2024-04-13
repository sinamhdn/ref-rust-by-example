#![allow(unused)]
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
    
            Guess { value }
        }
    
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    println!("Guess the number!");

    let secret_number:i32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number generated is {secret_number}");

    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        /*
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        */

        /*
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        */

        /*
        // not ideal because we run a big logic on every iteration instead add the variable check to a type
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        */

        let guess_i32: i32 = guess.trim().parse().expect("Please type a number!");
        let guess_i32: i32 = Guess::new(guess_i32).value(); // this is type checked value

        println!("Your guess is {guess}");

        /*
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        */
        match guess_i32.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
