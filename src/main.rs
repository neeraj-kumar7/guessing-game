use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("----------Guess the number!--------------");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");

    let mut number_of_guesses = 0;
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        number_of_guesses += 1;

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!!! Guesses taken: {number_of_guesses}");
                break;
            }
        }
    }
    
}