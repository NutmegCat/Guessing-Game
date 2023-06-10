use std::io;

fn main() {
    println!("\nGuess the number!");

    println!("\nPlease input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("\nGuess another number. ");
    let mut guess_again = String::new();

    io::stdin()
        .read_line(&mut guess_again)
        .expect("Failed to read line");

    println!("\nYour first guess was {guess}and your second guess was {guess_again}");
    
}