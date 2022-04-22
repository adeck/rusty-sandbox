
use rand::Rng;

use std::io;
use std::cmp::Ordering;



fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);
    println!("This is a guessing game.");
    'guessing: loop {
        println!("Secret num: {}", secret_num);
        let guess: u8 = match make_guess() {
            Some(v) => v,
            None => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low."),
            Ordering::Equal => break 'guessing,
            Ordering::Greater => println!("Too high."),
        }
    }
    println!("You guessed correctly!");
    println!("You won the game! Congratulations!");
}


fn make_guess() -> Option<u8> {
    println!("Make a guess!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error attempting to read line");
    match guess.trim().parse() {
        Ok(v) => if v < 101 {
            Some(v)
        } else {
            println!("Must guess a number in the range [1, 100].");
            None
        },
        Err(e) => {
            println!("Must guess a valid number. Error: {}", e);
            None
        },
    }
}

