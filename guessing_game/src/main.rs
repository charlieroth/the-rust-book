use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guesses = 0;

    println!("Guess the number");
    loop {
        num_guesses += 1;
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, try again"),
            Ordering::Greater => println!("Too big, try again"),
            Ordering::Equal => {
                println!("You win! You guessed {} times", num_guesses);
                break;
            }
        }
    }
}
