use std::env;

mod one;
mod two;

use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    match query.as_str() {
        "one" => one::day_one(),
        "two" => two::day_two(),
        _ => println!("Invalid query."),
    };
}

fn read_file(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => panic!("Error reading file. Make sure to add your puzzle input in a file called `input.txt` in the folder of the day you're choosing."),
    }
}
