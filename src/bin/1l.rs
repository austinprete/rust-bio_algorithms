//! ### Implement PatternToNumber
//!
//! Convert a DNA string to a number.
//!
//! **Given:** A DNA string Pattern.
//!
//! **Return:** `PatternToNumber(Pattern)`

extern crate bio_algorithms as bio;

use std::fs::File;
use std::io::prelude::*;

use bio::bio_types::DNA_Sequence;

fn main() {
    let mut f = File::open("test_files/1l.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern: DNA_Sequence = DNA_Sequence::from_string(lines[0]);

    println!("{}", pattern.pattern_to_number());
}
