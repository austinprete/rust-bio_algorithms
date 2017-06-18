use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

use bio_types::DNA_Sequence;

/// A k-mer is a string of length k. We define Count(Text, Pattern) as the number of times that
/// a k-mer Pattern appears as a substring of Text.
///
/// To compute Count(Text, Pattern), our plan is to “slide a window” down Text, checking whether
/// each k-mer substring of Text matches Pattern.
///
/// ```text
/// PatternCount(Text, Pattern)
///     count ← 0
///     for i ← 0 to |Text| − |Pattern|
///         if Text(i, |Pattern|) = Pattern
///             count ← count + 1
///     return count
/// ```
///
/// ### Implement PatternCount
///
/// **Given:** {DNA strings} Text and Pattern.
///
/// **Return:** Count(Text, Pattern)
pub fn problem_1a() {
    let mut f = File::open("test_files/1a.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let dna_pattern = DNA_Sequence::from_string(lines[0]);
    let sub_pattern = DNA_Sequence::from_string(lines[1]);

    let result = dna_pattern.pattern_match_count(sub_pattern);

    println!("{}", result);
}

/// We say that Pattern is a most frequent k-mer in Text if it maximizes Count(Text, Pattern)
/// among all k-mers. For example, "ACTAT" is a most frequent 5-mer in
/// "ACAACTATGCATCACTATCGGGAACTATCCT", and "ATA" is a most frequent 3-mer of "CGATATATCCATAG".
///
/// ### Frequent Words Problem
///
/// Find the most frequent k-mers in a string.
///
/// **Given:** A DNA string Text and an integer k.
///
/// **Return:** All most frequent k-mers in Text (in any order).
pub fn problem_1b() {
    let mut f = File::open("test_files/1b.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern = DNA_Sequence::from_string(lines[0]);
    let k = lines[1].parse::<usize>().unwrap();

    let result = find_frequent_words(&pattern, k);

    for word in result {
        print!("{} ", word);
    }
    println!("");
}

fn find_frequent_words(pattern: &DNA_Sequence, k: usize) -> Vec<DNA_Sequence> {

    let mut map = HashMap::new();
    let mut highest_count = 0;

    for index in 0..(pattern.len() - k + 1) {
        let sub_pattern = DNA_Sequence((&pattern[index..index + k]).to_vec());
        let count = map.entry(sub_pattern).or_insert(0);
        *count += 1;
        if *count > highest_count {
            highest_count = *count;
        }
    }

    (&map)
        .iter()
        .filter(|&(_, value)| *value == highest_count)
        .map(|(key, _)| key.clone())
        .collect()
}

/// ### Reverse Complement Problem
///
/// Find the reverse complement of a DNA string.
///
/// **Given:** A DNA string Pattern.
///
/// **Return:** Pattern, the reverse complement of Pattern.
pub fn problem_1c() {
    let mut f = File::open("test_files/1c.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern_string = lines[0];
    let mut dna_pattern = DNA_Sequence::from_string(pattern_string);

    println!("{}", dna_pattern.reverse_complement())
}

/// ### Pattern Matching Problem
///
/// Find all occurences of a pattern in a string
///
/// **Given:** Strings Pattern and Genome.
/// 
/// **Return:** All starting positions in Genome where Pattern appears as a substring.
pub fn problem_1d() {
    let mut f = File::open("test_files/1d.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern_string = lines[0];
    let dna_pattern = DNA_Sequence::from_string(pattern_string);

    let genome_string = lines[1];
    let genome_pattern = DNA_Sequence::from_string(genome_string);

    let matches = genome_pattern.find_pattern_matches(dna_pattern);

    for index in matches {
        print!("{} ", index);
    }

    println!("");
}

/// ### Implement PatternToNumber
///
/// Convert a DNA string to a number.
///
/// **Given:** A DNA string Pattern.
///
/// **Return:** `PatternToNumber(Pattern)`
pub fn problem_1l() {
    let mut f = File::open("test_files/1l.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)
        .expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern: DNA_Sequence = DNA_Sequence::from_string(lines[0]);

    println!("{}", pattern.pattern_to_number());
}

/// ### Implement NumberToPattern
///
/// Convert an integer to its corresponding DNA sequence.
///
/// **Given:** Integers index and k.
///
/// **Return:** `NumberToPattern(index, k)`
pub fn problem_1m() {
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
