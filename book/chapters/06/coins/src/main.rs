
fn main() {
    let q = Coin::Quarter(State::Alabama);
    println!("{:?} => {}", q, q.cents());
}

#[derive(Debug)]
enum Coin {
    Quarter(State),
    Dime,
    Nickel,
    Penny
}

impl Coin {
    fn cents(&self) -> u16 {
        match self {
            Self::Quarter(_) => 25,
            Self::Dime => 10,
            Self::Nickel => 5,
            Self::Penny => 1,
        }
    }
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Hawaii,
    Pennsylvania,
    Canada,
}

