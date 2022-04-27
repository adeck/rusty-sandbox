
use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);
    'guessing: loop {
        println!("Secret number: {}", secret_num);
        let guess = match make_guess() {
            Some(v) => v,
            None => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => break 'guessing,
            Ordering::Greater => println!("Too high"),
        }
    }
    println!("Correct!");
    println!("You won! Congratulations!");
}

fn make_guess() -> Option<u8> {
    println!("Make a guess!");
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u8 = match guess.trim().parse() {
        Ok(v) => v,
        Err(e) => {
            println!("Value not a valid number. Error was: {}", e);
            return None;
        },
    };
    if guess > 100 {
        println!("Value must be in range [1, 100]");
        return None;
    }
    return Some(guess);
}

