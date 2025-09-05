use std::io;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    println!("Enter your guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Falled to get your guess");

    println!("You guessed: {guess}");

}
