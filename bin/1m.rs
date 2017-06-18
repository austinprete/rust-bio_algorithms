//! ### Implement NumberToPattern
//!
//! Convert an integer to its corresponding DNA sequence.
//!
//! **Given:** Integers index and k.
//!
//! **Return:** `NumberToPattern(index, k)`

extern crate bio_algorithms as bio;

use std::fs::File;
use std::io::prelude::*;

use bio::bio_types::DNA_Sequence;

fn main() {
    let mut f = File::open("test_files/1m.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let number = lines[0].parse::<u64>().unwrap();
    let kmer_size = lines[1].parse::<u64>().unwrap();

    let result = DNA_Sequence::number_to_pattern(number, kmer_size);

    println!("{}", result);
}
