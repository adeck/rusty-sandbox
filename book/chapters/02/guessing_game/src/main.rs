
// use std::io;
use rand::Rng;


fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is: {}", secret_num);
}


