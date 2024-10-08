// The io library comes from the standard library, known as std
// https://crates.io/crates/rand

use rand::Rng; 
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // We use the let statement to create the variable

        io::stdin() //  std::io::stdin
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The colon (:) after guess tells Rust we’ll annotate the variable’s type.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
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
