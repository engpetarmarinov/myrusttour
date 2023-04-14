use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let from = 1;
    let to = 10;
    let secret_number = rand::thread_rng().gen_range(from..=to);
    let mut num_tries = 3;

    loop {
        println!("Please input your guess from {from} to {to}: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        num_tries -= 1;
        if num_tries <= 0 {
            println!("Nah, it was {secret_number}!");
            break;
        }
    }
}
