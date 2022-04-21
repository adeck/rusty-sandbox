
use std::io;
use rand::Rng;


fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is: {}", secret_num);
    println!("This is a guessing game!");
    println!("Make a guess...");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Must enter a valid string");
    println!("You guessed: {}", guess);
}


