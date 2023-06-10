use std::io;

fn main() {
    println!("\nGuess the number!");

    println!("\nPlease input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("\nFailed to read line");
    
    print!("\nYou guessed: {}\n", guess);   
}