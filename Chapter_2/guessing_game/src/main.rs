use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    
    let mut tries = 0;
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        match io::stdin().read_line(&mut guess) {
            Ok(0) => {
                println!("Ctrl-D detelcted.Exiting");
                break;
            }
            Ok(_) => {
                println!("OK value")
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }

        let num: i32 = match guess.trim().parse() {
            Ok(n @ 1..=10) => {
                tries += 1;
                n
            }
            Ok(_) => {
                println!("Please enter a number strictly between 1 and 10.(inclusive)");
                continue;
            }
            Err(non_number) => {
                println!("Enter a number {non_number}");
                continue;
            }
        };

        match num.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("In {} guesses", tries);
                println!("Got it");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }

        guess.clear();
    }
}
