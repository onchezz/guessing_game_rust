use rand::Rng;
use std::cmp::Ordering;
use std::io;

const TRIALS: u32 = 5;
fn main() {
    println!("Guess a number ");
    let mut left_trials = TRIALS;
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop {
        println!("the secret_number is {}", secret_number);
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                left_trials -= 1;
                println!("Too small!");
            }
            Ordering::Greater => {
                left_trials -= 1;
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        if left_trials == 0 {
            println!("You lose!");
            break;
        }
        println!("You guessed: {} ", &guess);
        println!("number of trials letf: {}", left_trials);
    }
}
