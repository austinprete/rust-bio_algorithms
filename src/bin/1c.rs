// Reverse Complement Problem
//
// Find the reverse complement of a DNA string.
//
// Given: A DNA string Pattern.
// Return: Pattern, the reverse complement of Pattern.

use std::fs::File;
use std::io::prelude::*;

mod bio_types;
use bio_types::DNA_Pattern;

fn main() {
    let mut f = File::open("test_files/1c.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text).expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern_string = lines[0];
    let mut dna_pattern = DNA_Pattern::from_string(pattern_string);

    println!("{}", dna_pattern.reverse_complement())
}

