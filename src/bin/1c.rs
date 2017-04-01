// Reverse Complement Problem
//
// Find the reverse complement of a DNA string.
//
// Given: A DNA string Pattern.
// Return: Pattern, the reverse complement of Pattern.

use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide {
    fn complement(&self) -> Nucleotide {
        use Nucleotide::*;

        match *self {
            A => T,
            C => G,
            G => C,
            T => A,
        }
    }

    fn from_char(letter: char) -> Nucleotide {
        use Nucleotide::*;

        match letter {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            _ => panic!("Incorrect lettter"),
        }
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Nucleotide::*;

        let letter = match *self {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
        };

        write!(f, "{}", letter)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
struct DNA_Pattern(Vec<Nucleotide>);

impl DNA_Pattern {
    fn reverse_complement(&mut self) -> DNA_Pattern {

        let reverse_complement_vec = self.0
            .iter()
            .rev()
            .map(|x| x.complement())
            .collect::<Vec<Nucleotide>>();

        DNA_Pattern(reverse_complement_vec)
    }

    fn from_string(dna_string: &str) -> DNA_Pattern {
        let pattern_vec = dna_string.chars()
            .map(|letter| Nucleotide::from_char(letter))
            .collect();

        DNA_Pattern(pattern_vec)
    }
}

impl fmt::Display for DNA_Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pattern_string = self.0
            .iter()
            .fold(String::new(),
                  |acc, &dna_letter| format!("{}{}", acc, dna_letter));

        write!(f, "{}", pattern_string)
    }
}

fn main() {
    let mut f = File::open("test_files/1c.txt").expect("Coudln't open file");
    let mut file_text = String::new();
    f.read_to_string(&mut file_text).expect("Couldn't read file");

    let lines: Vec<&str> = file_text.split('\n').collect();

    let pattern_string = lines[0];

    let mut dna_pattern = DNA_Pattern::from_string(pattern_string);

    println!("{}", dna_pattern.reverse_complement())
}

