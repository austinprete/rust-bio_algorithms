//! We say that Pattern is a most frequent k-mer in Text if it maximizes Count(Text, Pattern)
//! among all k-mers. For example, "ACTAT" is a most frequent 5-mer in
//! "ACAACTATGCATCACTATCGGGAACTATCCT", and "ATA" is a most frequent 3-mer of "CGATATATCCATAG".
//!
//! ### Frequent Words Problem
//!
//! Find the most frequent k-mers in a string.
//!
//! **Given:** A DNA string Text and an integer k.
//!
//! **Return:** All most frequent k-mers in Text (in any order).

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

mod bio_types;
use bio_types::DNA_Pattern;

fn main() {
    let mut f = File::open("test_files/1b.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text).expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern = DNA_Pattern::from_string(lines[0]);
    let k = lines[1].parse::<usize>().unwrap();

    let result = find_frequent_words(pattern, k);

    for word in result {
        print!("{} ", word);
    }
    println!("");
}

fn find_frequent_words(pattern: DNA_Pattern, k: usize) -> Vec<DNA_Pattern> {

    let mut map = HashMap::new();
    let mut highest_count = 0;

    for index in 0..(pattern.len() - k + 1) {
        let sub_pattern = DNA_Pattern((&pattern[index..index + k]).to_vec());
        let count = map.entry(sub_pattern).or_insert(0);
        *count += 1;
        if *count > highest_count {
            highest_count = *count;
        }
    }

    let frequent_words = (&map)
        .iter()
        .filter(|&(_, value)| *value == highest_count)
        .map(|(key, _)| key.clone())
        .collect();

    frequent_words
}
