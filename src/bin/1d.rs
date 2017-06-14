//! ### Pattern Matching Problem
//!
//! Find all occurences of a pattern in a string
//!
//! **Given:** Strings Pattern and Genome.
//! 
//! **Return:** All starting positions in Genome where Pattern appears as a substring.

extern crate bio_algorithms as bio;

use std::fs::File;
use std::io::prelude::*;

use bio::bio_types::DNA_Sequence;

fn main() {
    let mut f = File::open("test_files/1d.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern_string = lines[0];
    let dna_pattern = DNA_Sequence::from_string(pattern_string);

    let genome_string = lines[1];
    let genome = DNA_Sequence::from_string(genome_string);

    let match_occurences = genome.find_occurences_of_pattern(dna_pattern);

    for index in match_occurences {
        print!("{} ", index);
    }
    println!();
}
