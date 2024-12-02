// mod one;
mod two;

use std::fs;

fn main() {
    // one::day_one();
    two::day_two();
}

fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
