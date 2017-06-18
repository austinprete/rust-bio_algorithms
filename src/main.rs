extern crate bio_algorithms;

use std::env;

use bio_algorithms::solutions::chapter_1::*;

fn main() {
    let problem_number = env::args().skip(1).next();
    match problem_number {
        Some(number) => {
            match number.trim().as_ref() {
                "1a" => problem_1a(),
                "1b" => problem_1b(),
                "1c" => problem_1c(),
                "1d" => problem_1d(),
                "1l" => problem_1l(),
                "1m" => problem_1m(),
                _ => println!("Unexpected problem number"),
            }
        }
        None => println!("Please enter a problem number as an argument"),
    }

}