use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");
    let mut rng = rand::rng();
    let secret_number: u32 = rng.random_range(1..=100);
    // E0425: rand::thread_rng() is not used with rand 0.10.2.
    // E0599: random_range needs the RngExt trait in scope.
    loop {
        println!("Enter your Guess");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Input");
        let guess: u32 = guess.trim().parse().expect("Please Enter a Number");
        println!("You Guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        };
    }
}
