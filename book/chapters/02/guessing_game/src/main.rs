
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);
    println!("This is a guessing game!");
    while !correctly_guessed(secret_num) {}
    println!("You won the game! Congratulations!");
}

/// Returns true IFF the user correctly guesses the secret number and no error occurs reading stdin.
fn correctly_guessed(secret_num: u8) -> bool {
    println!("Secret number is: {}", secret_num);
    println!("Make a guess...");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Must enter a valid string");
    let guess: u8 = match guess.trim().parse() {
        Err(e) => {
            println!("Enter a valid number. Error: {}", e);
            return false;
        },
        Ok(v) => v,
    };
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too low"),
        Ordering::Equal => return true,
        Ordering::Greater => println!("Too high"),
    }
    return false;
}


