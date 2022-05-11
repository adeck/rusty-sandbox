
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    or_else_what();
}

fn or_else_what() {
    let filename = "hello.txt";
    let f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() != ErrorKind::NotFound {
            panic!("Unable to open file. Error: {:?}", error);
        }
        println!("File not found. Creating...");
        let result = File::create(filename).unwrap_or_else(|error| {
            panic!("Unable to create file. Error: {:?}", error);
        });
        println!("File created.");
        return result;
    });
    println!("File found!");
    
}


fn matchy_matchy() {
    let filename = "hello.txt";
    let f = match File::open(filename) {
        Ok(f) => {
            println!("Found file!");
            f
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(f) => {
                    println!("Created file!");
                    f
                },
                Err(e) => {
                    panic!("Error creating file '{}': {:?}", filename, e);
                },
            },
            other_error => {
                panic!("Error opening file '{}': {:?}", filename, other_error);
            },
        },
    };
}
