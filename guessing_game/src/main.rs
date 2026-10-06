use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // TODO: What is the difference between `low..high` and `low..=high`?
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        // NOTE: Skip to the next loop if we input something that isn't a number.
        let guess: u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
