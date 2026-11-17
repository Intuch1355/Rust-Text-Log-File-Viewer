
mod ui;
mod unit_test;

use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let keyword = &args[2];

    // read file
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            println!("error: {}", error);
            return;
        }
    };

    // keyword
    let count = content.matches(keyword).count();
    println!("{}", count)
}
