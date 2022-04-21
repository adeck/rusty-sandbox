
use rand::Rng;

use std::io;
use std::cmp::Ordering;


fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..101);
    println!("This is a guessing game!");
    'guessing: loop {
        println!("The secret number is: {}", secret_num);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read string input.");
        let guess: u8 = match guess.trim().parse() {
            Err(e) => {
                println!("Must enter valid number. Error: {}", e);
                continue 'guessing;
            },
            Ok(v) => {
                if v < 101 { v } else {
                    println!("Value {} too high. Must be in range [0, 100].", v);
                    continue 'guessing;
                }
            },
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("Correct!");
                break 'guessing;
            },
            Ordering::Greater => println!("Too high"),
        }
    }
    println!("You won the game! Congratulations!");
}

