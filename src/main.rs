use std::env;

mod dec_1;
mod dec_2;
mod dec_3;
mod dec_4;

use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    match query.as_str() {
        "one" => dec_1::day_one(),
        "two" => dec_2::day_two(),
        "three" => dec_3::day_three(),
        "four" => dec_4::day_four(),
        _ => println!("Invalid query."),
    };
}

fn read_file(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => panic!("Error reading file. Make sure to add your puzzle input in a file called `input.txt` in the folder of the day you're choosing."),
    }
}
