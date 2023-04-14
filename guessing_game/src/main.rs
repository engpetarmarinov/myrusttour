use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let from = 1;
    let to = 10;
    let secret_number = rand::thread_rng().gen_range(from..=to);
    println!("Please input your guess from {from} to {to}.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! It was {secret_number}."),
        Ordering::Greater => println!("Too big! It was {secret_number}."),
        Ordering::Equal => println!("You win!"),
    }
}
