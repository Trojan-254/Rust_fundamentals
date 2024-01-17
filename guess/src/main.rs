// Guessing game

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // The main function 
    println!("============== Welcome to the guessing game ============");
    println!("**************    Hope you like it here ***********");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Ooops, too big"),
            Ordering::Equal => {println!("Hurray You win"); break;},
            Ordering::Less => println!("Ooops, too small")
        }
    }
}