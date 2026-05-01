use std::io;
use std::cmp::Ordering;
use rand::RngExt;
use colored::*;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input you guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("{}", "Too low!".red()),
            Ordering::Greater => println!("{}", "Too high!".red()),
            Ordering:: Equal => 
            {
                println!("{}", "Correct!".green());
                break;
            }
        }
    }

}
