use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number");
    let mut guesses: u8 = 0;
    // generate random number
    let secret_number = rand::rng().random_range(1..=100);
    let x = "opppppp";
    println!("{x}");

    loop {
        println!("Enter an integer");
        let mut guess = String::new();
        guesses += 1;

        // read user input
        io::stdin().read_line(&mut guess).expect("invalid input");

        // parse the integer from the input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare to the generated number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is lower than the answer"),
            Ordering::Greater => println!("Your guess is greater than the answer"),
            Ordering::Equal => {
                println!("Yayyyy! You got the answer correctly");
                println!("You took {} guesses", guesses);
                break;
            }
        }
    }
}
