use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let min_num = 1;
    let max_num = 100;

    let secret_number = rand::thread_rng().gen_range(min_num, max_num + 1);

    let mut num_guesses: u32 = 0;

    let mut lower_bound = min_num;
    let mut upper_bound = max_num;

    loop {
        println!(
            "Please input your guess between {} and {}.",
            lower_bound, upper_bound
        );
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // increment number of guesses after validating input
        num_guesses = num_guesses + 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                if lower_bound <= guess {
                    lower_bound = guess + 1;
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                if upper_bound >= guess {
                    upper_bound = guess - 1;
                }
            }
            Ordering::Equal => {
                println!(
                    "Well done! It took you {} guesses to find the secret number {}.",
                    num_guesses, secret_number
                );
                break;
            }
        }

        if upper_bound - lower_bound == 0 {
            println!(
                "Well done! It took you {} guesses to find the secret number {}.",
                num_guesses, secret_number
            );
            break;
        }
    }
}
