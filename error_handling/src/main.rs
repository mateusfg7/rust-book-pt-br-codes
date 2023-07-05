use std::fs;
use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // panic!("Crash and burn!");

    // let v = vec![1, 2, 3];
    // v[99];

    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Problem creating file: {}", err),
    //         },
    //         other_error => panic!("Problem opening file: {}", other_error),
    //     },
    // };

    // Returns the Ok value and panic at error with generic error message and the kind of error enum
    // let _greeting_file = File::open("hello.txt").unwrap();
    //
    // Returns the Ok value and panic at error with a given error message and the ind of error enum
    // let _greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// THE SAME FUNCTION, BUT WITH ERROR PROPAGATION `?`.,

fn _read_username_from_file_with_propagation() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// THE SAME FUNCTION, BUT SHORTENED,

fn _read_username_from_file_with_propagation_shortened() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// I'M NOT SATISFIED, LET'S SHORTEN IT!!!

fn _read_username_from_file_with_propagation_shortened_increased() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
