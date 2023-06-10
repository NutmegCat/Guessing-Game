use std::io; // standard library crate
use rand::Rng; // imported crate for random number generation
use std::cmp::Ordering; // standard library crate for comparison

fn main() {
    println!("\nGuess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number between 1 and 100

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin() // standard library crate for input
            .read_line(&mut guess)
            .expect("\nFailed to read line");

        let guess: u32 = match guess.trim().parse(){ // convert string to u32
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("\nYou guessed: {guess}\n");
 
        match guess.cmp(&secret_number) { // compare guess with secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\n"); // print win message    
                break;
            }
        }
    }
}