
use std::io;
use rand::Rng;


fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);
    println!("This is a guessing game!");
    'guessing: loop {
        println!("Secret number is: {}", secret_num);
        println!("Make a guess...");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Must enter a valid string");
        let guess: u8 = match guess.trim().parse() {
            Err(e) => {
                println!("Enter a valid number. Error: {}", e);
                continue 'guessing;
            },
            Ok(v) => v,
        };
        println!("You guessed: {}", guess);
    }
}


