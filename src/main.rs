use std::io;

fn main() {
    println!("\nGuess the number!");

    println!("\nPlease input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    print!("You guessed: {}\n", guess);   
}