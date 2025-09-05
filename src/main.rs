use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=100);
    // println!("Secret number is: {secret_number}");


    let mut guess = String::new();

    println!("Enter your guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Falled to get your guess");

    println!("You guessed: {guess}");

}
