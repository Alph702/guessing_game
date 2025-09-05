use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Enter your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Falled to get your guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
