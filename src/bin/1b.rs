// We say that Pattern is a most frequent k-mer in Text if it maximizes Count(Text, Pattern) 
// among all k-mers. For example, "ACTAT" is a most frequent 5-mer in "ACAACTATGCATCACTATCGGGAACTATCCT", 
// and "ATA" is a most frequent 3-mer of "CGATATATCCATAG".
// 
// Frequent Words Problem
// 
// Find the most frequent k-mers in a string.
// 
// Given: A DNA string Text and an integer k.
// 
// Return: All most frequent k-mers in Text (in any order).

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("test_files/1b.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text).expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let text = lines[0];
    let k = lines[1].parse::<usize>().unwrap();

    let result = find_frequent_words(text, k);

    for word in result {
        print!("{} ", word);
    }
    println!("");
}

fn find_frequent_words(text: &str, k: usize) -> Vec<&str> {
    let mut map = HashMap::new();
    let mut highest_count = 0;
    let mut frequent_words = Vec::new();

    for index in 0..(text.len()-k+1) {
        let substring = &text[index..index+k];
        let count = map.entry(substring).or_insert(0);
        *count += 1;
        if *count > highest_count {
            highest_count = *count;
        }
    }

    for (key, value) in &map {
        if *value == highest_count {
            frequent_words.push(*key);
        }
    }

    frequent_words
}