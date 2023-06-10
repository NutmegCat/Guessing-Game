use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\nGuess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("\nPlease input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("\nFailed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("\nYou guessed: {guess}\n");
 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!\n"),
        Ordering::Greater => println!("Too big!\n"),
        Ordering::Equal => println!("You win!\n"),
    }
}