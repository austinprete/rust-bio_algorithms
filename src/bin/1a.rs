//! A k-mer is a string of length k. We define Count(Text, Pattern) as the number of times that
//! a k-mer Pattern appears as a substring of Text.
//!
//! To compute Count(Text, Pattern), our plan is to “slide a window” down Text, checking whether
//! each k-mer substring of Text matches Pattern.
//!
//! ```text
//! PatternCount(Text, Pattern)
//!     count ← 0
//!     for i ← 0 to |Text| − |Pattern|
//!         if Text(i, |Pattern|) = Pattern
//!             count ← count + 1
//!     return count
//! ```
//!
//! ### Implement PatternCount
//!
//! **Given:** {DNA strings} Text and Pattern.
//!
//! **Return:** Count(Text, Pattern)

use std::io::prelude::*;
use std::fs::File;

mod bio_types;
use bio_types::DNA_Sequence;

fn main() {
    let mut f = File::open("test_files/1a.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text).expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let dna_pattern = DNA_Sequence::from_string(lines[0]);
    let sub_pattern = DNA_Sequence::from_string(lines[1]);

    let result = dna_pattern.pattern_match_count(sub_pattern);

    println!("{}", result);
}
