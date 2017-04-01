// A k-mer is a string of length k. We define Count(Text, Pattern) as the number of times that a k-mer Pattern appears as a 
// substring of Text. For example,
// 
// Count(ACAACTATGCATACTATCGGGAACTATCCT,ACTAT)=3Count(ACAACTATGCATACTATCGGGAACTATCCT,ACTAT)=3.
// We note that Count(CGATATATCCATAGCGATATATCCATAG, ATAATA) is equal to 3 (not 2) since we should account for overlapping 
// occurrences of Pattern in Text.
// 
// To compute Count(Text, Pattern), our plan is to “slide a window” down Text, checking whether each k-mer substring of 
// Text matches Pattern. 
// 
//     PatternCount(Text, Pattern)
//         count ← 0
//         for i ← 0 to |Text| − |Pattern|
//             if Text(i, |Pattern|) = Pattern
//                 count ← count + 1
//         return count
// 
// Implement PatternCount
// 
// Given: {DNA strings}} Text and Pattern.
// Return: Count(Text, Pattern).

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("test_files/1a.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text).expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let result = pattern_count(lines[0], lines[1]);

    println!("{}", result);
}

fn pattern_count(text: &str, pattern: &str) -> u64 {
    let mut count = 0;

    for index in 0..(text.len() - pattern.len() + 1) {
        if pattern == &text[index..index + pattern.len()] {
            count += 1;
        }
    }

    count
}

