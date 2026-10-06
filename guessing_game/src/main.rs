use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // TODO: What is the difference between `low..high` and `low..=high`?
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    // TODO: Currently accepts all input without validation.
    // add a retry loop which validates the input is a number.
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("You got it!"),
    }
}
