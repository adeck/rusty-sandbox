
use std::io;

mod company;
use company::Company;

mod command;

fn main() {
    let mut us = Company::new();
    'repl: loop {
        let line = read_line();
        let parts: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
        println!("The parts were: {:?}", parts);
        command::Command::parse(parts);
        // TODO -- read user input
        // TODO -- attempt to parse user input
        // TODO -- act on relevant command
    }
}

/// Reads a line from stdin, trims and lowercases that line
fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line properly. Exiting...");
    line.trim().to_lowercase()
}
